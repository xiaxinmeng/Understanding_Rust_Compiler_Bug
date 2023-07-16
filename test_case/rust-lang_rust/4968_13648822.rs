
fn main() {
    let a_tuple = (1, 2);     // Introduces new local variable 'a_tuple'
    match 0 {
        a_tuple => {          // Matches every possible value and binds them to a new local variable 'a_tuple'
            io::println("A"); // That variable never gets used here, hence the warning
        }
        _ => {                // Because the above arm matches everything, this arm can never be reached
            io::println("B");
        }
    }
}
