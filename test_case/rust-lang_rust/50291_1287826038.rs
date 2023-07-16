

struct Complex(Box<dyn Fn(i32) -> i32>);
impl Termination for Complex {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}
#[test]
#[allow(dead_code)]
fn returns_closure() -> Complex {
    Complex(Box::new(|x| x + 1))
}
