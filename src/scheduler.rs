use chrono::TimeZone;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Scheduler {
    states: Arc<Mutex<Vec<JobState>>>,
    job_thread: Option<thread::JoinHandle<()>>,
}

pub struct JobDefinition {
    pub f: Arc<Mutex<dyn (Fn(RunContext)) + Send + 'static>>,
    pub interval: chrono::Duration,
    pub name: String,
}

struct JobState {
    def: JobDefinition,
    last_finish: chrono::DateTime<chrono::Utc>,
    force_run: bool,
}

pub struct RunContext {}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            job_thread: None,
            states: Arc::new(Mutex::new(vec![]))
        }
    }

    pub fn add(&mut self, def: JobDefinition) {
        self.states.lock().unwrap().push(JobState {
            def,
            last_finish: chrono::Utc.yo(1970, 1).and_hms(0,0,0),
            force_run: false,
        });
    }

    pub fn force_run(&mut self, job_name: &str) {
        let mut states = self.states.lock().unwrap();
        let j = states
                    .iter_mut().find(|j| j.def.name == job_name)
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
    fn run(states: Arc<Mutex<Vec<JobState>>>) {
        loop {
            {
                for s in states.lock().unwrap().iter_mut() {
                    if (s.last_finish + s.def.interval < chrono::Utc::now())
                        || s.force_run
                    {
                        (s.def.f.lock().unwrap())(RunContext {});
                        s.last_finish = chrono::Utc::now();
                        s.force_run = false;
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}
