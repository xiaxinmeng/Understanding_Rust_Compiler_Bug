rust
fn main() {
    let ability: Result<u8, u8> = Ok(8);
    let mut maybe_cost;
    if let Ok(cost) = ability {
        maybe_cost = Some(cost);
    } else {
        // Comment out this line to fix warning:
        panic!();
    }
    let cost = maybe_cost.unwrap();
    
    println!("{:?}", cost);
}
