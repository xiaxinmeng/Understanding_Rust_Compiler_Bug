rust
fn main() {
    println!("hardware threads {}", num_cpus::get());
    println!("hardware cores {}", num_cpus::get_physical());
}
