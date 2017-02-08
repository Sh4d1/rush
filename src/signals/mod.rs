pub mod sigint;
pub mod sigtstp;

pub fn init_signals() {
    sigint::init_sigint();
    sigtstp::init_sigtstp();
    //sigcont::init_sigcont();
}
