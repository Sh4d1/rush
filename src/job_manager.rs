use std::sync::Mutex;

lazy_static! {
    pub static ref JOB: Mutex<JobManager> = Mutex::new(JobManager::new());
}

pub struct JobManager {
    list: Vec<(i32, String)>,
    size: u32,
}

impl JobManager {
    pub fn new() -> JobManager {
        let mut list = Vec::new();
        list.push((-1, "".to_string()));
        JobManager { list: list, size: 0}

    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn push(&mut self, pid: i32, name: String) {
        self.size += 1;
        self.list.push((pid, name));
    }

    pub fn pop(&mut self) -> (i32, String) {
        self.size -= 1;
        self.list.pop().expect("Should not pop empty job")
    }

    pub fn set_active(&mut self, pid: i32, name: String) {
        self.list[0] = (pid, name);
    }

    pub fn get_active(&mut self) -> (i32, String) {
        self.list[0].to_owned()
    }

}
