rust
trait Testable {
     // panic on failure
     fn run(&self);
     fn name(&self) -> String;
     fn is_bench(&self) -> bool;
}
