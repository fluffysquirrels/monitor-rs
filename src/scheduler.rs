use std::{
    collections::BTreeMap,
    sync::{Arc, mpsc, Mutex},
    thread,
};

pub struct Scheduler {
    states: Arc<Mutex<BTreeMap<String, JobState>>>,
    job_thread: Option<JobThread>,
}

struct JobThread {
    handle: thread::JoinHandle<()>,
    tx: mpsc::SyncSender<ThreadMessage>,
}

pub struct JobDefinition {
    pub f: Arc<Mutex<dyn (Fn(RunContext)) + Send + 'static>>,
    pub interval: chrono::Duration,
    pub name: String,
}

struct JobState {
    def: JobDefinition,
    last_finish: Option<chrono::DateTime<chrono::Utc>>,
    force_run: bool,
}

pub struct RunContext {}

enum ThreadMessage {
    Shutdown,
    ForceRun,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            job_thread: None,
            states: Arc::new(Mutex::new(BTreeMap::new()))
        }
    }

    pub fn add(&mut self, def: JobDefinition) {
        self.states.lock().unwrap().insert(
            def.name.clone(),
            JobState {
                def,
                last_finish: None,
                force_run: false,
            });
    }

    pub fn force_run(&mut self, job_name: &str) {
        let mut states = self.states.lock().unwrap();
        let j = states
                    .get_mut(job_name)
                    .expect("To find the job");
        j.force_run = true;
        if let Some(t) = self.job_thread.as_ref() {
            match t.tx.send(ThreadMessage::ForceRun) {
                Err(_) =>
                    error!("Error sending scheduler::ThreadMessage::ForceRun, remote end is dropped"),
                Ok(_) => (),
            }
        };
    }

    pub fn spawn(&mut self) {
        assert!(self.job_thread.is_none());
        let sc = self.states.clone();
        let (tx, rx) = mpsc::sync_channel::<ThreadMessage>(5);
        let handle = thread::Builder::new().name("scheduler".to_owned())
                        .spawn(move || {
                            Self::run(sc, rx)
                        }).unwrap();
        self.job_thread = Some(JobThread {
            handle,
            tx,
        });
    }

    pub fn join(&mut self) {
        assert!(self.job_thread.is_some());
        let t = self.job_thread.take().unwrap();
        match t.tx.send(ThreadMessage::Shutdown) {
            Err(_) =>
                error!("Error sending scheduler::ThreadMessage::Shutdown, remote end is dropped"),
            Ok(_) => (),
        };
        match t.handle.join() {
            Err(e) => error!("Error joining scheduler thread: {:?}", e),
            Ok(_) => (),
        };
    }

    // TODO: Exit condition.
    fn run(states: Arc<Mutex<BTreeMap<String, JobState>>>, rx: mpsc::Receiver<ThreadMessage>) {
        loop {
            // The goal is to lock `states` for multiple short
            // durations rather than one long duration (including the
            // work function), which would potentially block the UI
            // for a noticable period.

            // First collect the names of the jobs that are eligible to run.
            // TODO: This could use a heap.
            let to_run = states.lock().unwrap()
                .values_mut()
                .filter(|s| {
                    s.last_finish.is_none()
                        || (s.last_finish.unwrap() + s.def.interval < chrono::Utc::now())
                        || s.force_run
                }).map(|s| s.def.name.clone())
                .collect::<Vec<String>>();

            // Then iterate through the jobs to run.
            for name in to_run.iter() {
                // Lock to get the job work function.
                let f = match states.lock().unwrap().get(name) {
                    // Didn't find the job, so it was removed, continue with the next job.
                    None => continue,
                    Some(s) => s.def.f.clone(),
                };
                // f runs without locking.
                (f.lock().unwrap())(RunContext {});
                {
                    // Lock `states` to write the results.
                    let mut states_lock = states.lock().unwrap();
                    let mut s = match states_lock.get_mut(name) {
                        // Didn't find the job, so it was removed, continue with the next job.
                        None => continue,
                        Some(s) => s
                    };

                    s.last_finish = Some(chrono::Utc::now());
                    s.force_run = false;
                }
            }

            // Wait a short period before evaluating the jobs again to check what to run.
            let res = rx.recv_timeout(std::time::Duration::from_secs(1));
            match res {
                Ok(msg) => match msg {
                    ThreadMessage::Shutdown => return,
                    ThreadMessage::ForceRun => (),
                },
                Err(mpsc::RecvTimeoutError::Timeout) => (),
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Scheduler;

    #[test]
    fn join() {
        let mut s = Scheduler::new();
        s.spawn();
        s.join();
    }
}
