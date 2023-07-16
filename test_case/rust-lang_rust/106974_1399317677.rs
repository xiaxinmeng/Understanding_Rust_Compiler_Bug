rust

fn main() {
    let mut v1: [Vec<i32>; 1] = [vec![1, 2, 3]];

    loop {
        let go = || {
            if v1[0][1] == 2 {
                println!("ok");
            }
        };
        
        if v1.len() >= 1 {
            return go();
        }
        for i in v1.iter_mut() {
            println!("{i:?}");
        }
    }
}
