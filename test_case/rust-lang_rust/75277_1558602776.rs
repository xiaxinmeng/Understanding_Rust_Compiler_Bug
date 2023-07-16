rust
struct Magic(u32);

fn fully_fleshed_out_function(magic: Magic) -> Option<u32> {
    Some(magic.0)
}

pub fn main() {
    #![allow(unreachable_code)]
    
    // Gosh, I don't know how to make one of these yet.
    let magic: Magic = todo!();
 
    // But I know what to DO with it!
    let result = fully_fleshed_out_function(magic);

    println!("Result: {result:?}");
}
