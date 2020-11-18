use crate::BoxError;
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
        if let Some(t) = self.job_thread.as_ref() {
            if t.tx.send(ThreadMessage::ForceRun).is_err() {
                error!("Error sending scheduler::ThreadMessage::ForceRun, remote end is dropped");
            }
        };
    }

    pub fn force_run(&mut self, job_name: &str) -> Result<(), BoxError> {
        let mut states = self.states.lock().unwrap();
        let j = states.get_mut(job_name)
                      .ok_or_else(|| format!("Didn't find the scheduler job '{}'", job_name))?;
        j.force_run = true;
        if let Some(t) = self.job_thread.as_ref() {
            if t.tx.send(ThreadMessage::ForceRun).is_err() {
                error!("Error sending scheduler::ThreadMessage::ForceRun, remote end is dropped");
            }
        };
        Ok(())
    }

    pub fn spawn(&mut self) {
        assert!(self.job_thread.is_none());
        let sc = self.states.clone();
        let (tx, rx) = mpsc::sync_channel::<ThreadMessage>(5);
        let handle = thread::Builder::new().name("monitor-scheduler".to_owned())
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
        if t.tx.send(ThreadMessage::Shutdown).is_err() {
            error!("Error sending scheduler::ThreadMessage::Shutdown, remote end is dropped");
        };
        if let Err(e) = t.handle.join() {
            error!("Error joining scheduler thread: {:?}", e);
        };
    }

    fn run(states: Arc<Mutex<BTreeMap<String, JobState>>>, mut rx: mpsc::Receiver<ThreadMessage>) {
        loop {
            // The goal is to lock `states` for multiple short
            // durations rather than one long duration (including the
            // work function), which would potentially block the UI
            // for a noticable period.

            // First collect the names of the jobs that are eligible to run.
            // TODO: This could use a heap.
            let to_run = states.lock().unwrap()
                .values()
                .filter(|s| {
                    s.last_finish.is_none()
                        || (s.last_finish.unwrap() + s.def.interval < chrono::Utc::now())
                        || s.force_run
                }).map(|s| s.def.name.clone())
                .collect::<Vec<String>>();

            // Then iterate through the jobs to run.
            for name in to_run.iter() {
                trace!("Running job `{}'", name);
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
            let wait_res = Self::run_wait(&mut rx, std::time::Duration::from_secs(1));
            match wait_res.flow_control {
                FlowControl::Exit => return,
                FlowControl::Continue => (),
            };

            if wait_res.wait_duration.num_seconds() >= 10 {
                info!("Resume from suspend detected");
                // Wait a bit longer to give time for the network to come back up
                // after suspending.

                let wait_res = Self::run_wait(&mut rx, std::time::Duration::from_secs(5));
                match wait_res.flow_control {
                    FlowControl::Exit => return,
                    FlowControl::Continue => (),
                };
            }
        }
    }

    fn run_wait(rx: &mut mpsc::Receiver<ThreadMessage>,
                desired_sleep_duration: std::time::Duration
    ) -> WaitResult {
        let before_wait = chrono::Utc::now();
        trace!("Scheduler thread sleeping ...");
        let res = rx.recv_timeout(desired_sleep_duration);
        let after_wait = chrono::Utc::now();
        let wait_duration = after_wait - before_wait;
        trace!("Scheduler thread slept wait_duration={}ms", wait_duration.num_milliseconds());

        let flow_control = match res {
            Ok(msg) => match msg {
                ThreadMessage::Shutdown => FlowControl::Exit,
                ThreadMessage::ForceRun => FlowControl::Continue,
            },
            Err(mpsc::RecvTimeoutError::Timeout) => FlowControl::Continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => FlowControl::Exit,
        };

        WaitResult {
            wait_duration,
            flow_control,
        }
    }
}

impl Default for Scheduler {
    fn default() -> Scheduler {
        Scheduler::new()
    }
}

struct WaitResult {
    wait_duration: chrono::Duration,
    flow_control: FlowControl,
}

enum FlowControl {
    Exit,
    Continue,
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
