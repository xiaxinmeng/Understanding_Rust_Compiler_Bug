rust
fn main() {
    'outer: loop {
        macro_rules! exit_loop { () => { break 'outer; } }
        loop {
            println!("Start of inner");
            None.ok_or_else(||{exit_loop!();});
           // println!("Not reachable");
        }
       // println!("End of outer");
    }
    println!("End of main");
}
