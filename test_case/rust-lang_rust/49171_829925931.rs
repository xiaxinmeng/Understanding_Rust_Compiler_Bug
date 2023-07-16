
fn main() {

    let mut ff_loop_abort = false;
    
    loop {  // Resize loop
        ff_loop_abort = false;
        
        loop {  // Rendering loop
            if true {  // A: Standin for complex conditional
                ff_loop_abort = true;
                break;
            }
        }
    
        if ff_loop_abort {
            if true {  // B: Standin for complex conditional
                break;
            }
        }
    }
    
    if ff_loop_abort {
        println!("loop aborted abnormally");
    }
    
}
