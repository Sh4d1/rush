use job_manager::JOB;


pub fn jobs(args: String) -> i8 {
    let mut err_code = 0;
    let size = args.split_whitespace().count();

    match size {
        0 => {
            if JOB.lock().unwrap().get_size() == 0 {
                println!("jobs: No jobs running");
                err_code = 1;
            } else {
                JOB.lock().unwrap().print();
            }
        }
        _ => {
            err_code = 1;
            println!("Too many arguments for jobs command");
        },
    }
    err_code
}
