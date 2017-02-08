use std::sync::Mutex;

lazy_static! {
    pub static ref JOB: Mutex<JobManager> = Mutex::new(JobManager::new());
}

#[derive(Debug)]
pub enum State {
    Running,
    Stopped
}

pub struct JobManager {
    list: Vec<(i32, String, State)>,
    active: (i32, String)
}

impl JobManager {
    pub fn new() -> JobManager {
        JobManager { list: Vec::new(), active: (-1, "".to_string())}
    }

    pub fn get_size(&self) -> i32 {
        self.list.len() as i32
    }


    pub fn push(&mut self, pid: i32, name: String, state: State) {
        self.list.push((pid, name, state));
    }

    pub fn pop(&mut self) -> (i32, String, State) {
        self.list.pop().expect("Should not pop empty job")
    }

    pub fn pop_stopped(&mut self) -> Option<(i32, String, State)> {
        let mut i = 0;
        for job in self.list.iter() {
            match job {
                &(_, _, State::Stopped) => break,
                _ => i += 1,
            }
        }
        if i < self.list.len() {
            Some(self.list.remove(i))
        } else {
            None
        }
    }


    pub fn set_active(&mut self, pid: i32, name: String) {
        self.active = (pid, name);
    }

    pub fn get_active(&mut self) -> (i32, String) {
        self.active.to_owned()
    }

    pub fn print(&self) {

        println!("Job   GPID    Name     State");
        let mut n = 1;
        for a in &(self.list) {
            println!("{:<6}{:<8}{:<8} {:?}", n, a.0, a.1, a.2);
            n+=1;
        }
    }

}
