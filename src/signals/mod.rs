pub mod sigint;
pub mod sigtstp;
pub mod sigttou;

pub fn init_signals() {
    sigint::init_sigint();
    sigtstp::init_sigtstp();
    sigttou::init_sigttou();
    //sigcont::init_sigcont();
}
