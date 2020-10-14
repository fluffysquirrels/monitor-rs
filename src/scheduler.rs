use chrono::TimeZone;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Scheduler {
    states: Arc<Mutex<BTreeMap<String, JobState>>>,
    job_thread: Option<thread::JoinHandle<()>>,
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
    }

    pub fn spawn(&mut self) {
        assert!(self.job_thread.is_none());
        let sc = self.states.clone();
        self.job_thread = Some(
            thread::Builder::new().name("scheduler".to_owned())
                .spawn(move || {
                    Self::run(sc)
                }).unwrap()
        );
    }

    //         pub fn join(&mut self) {
    //             assert!(self.job_thread.is_some());
    //             let t = self.job_thread.take();
    //             set_exit_condition();
    //             t.join();
    //         }

    // TODO: Exit condition.
    // TODO: Don't lock the states for so long.
    fn run(states: Arc<Mutex<BTreeMap<String, JobState>>>) {
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
            thread::sleep(Duration::from_secs(1));
        }
    }
}
