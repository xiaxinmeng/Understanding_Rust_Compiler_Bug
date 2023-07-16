
extern crate native;

fn main() {
    native::task::spawn(proc() customtask());
}

fn customtask() {
    let mut timer = std::io::timer::Timer::new().unwrap();
    let periodic = timer.periodic(100);
    periodic.recv();
}
