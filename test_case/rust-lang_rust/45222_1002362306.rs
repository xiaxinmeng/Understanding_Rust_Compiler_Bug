rust
pub fn example(m: i32, max: i32) -> i32 {
    for i in 1..(max + 1) {
        if i * i == m { 
            return i;
        }   
    }   
    0   
}

pub fn example_inc(m: i32, max: i32) -> i32 {
    for i in 1..=max {
        if i * i == m { 
            return i;
        }   
    }   
    0   
}
