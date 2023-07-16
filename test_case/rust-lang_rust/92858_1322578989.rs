rust
#[derive(Debug)]
struct SomeStruct {
    val: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deconstruct_move() {
        let mut maybe_s = Some(SomeStruct { val: 0 });
        let mut maybe_t = Some(SomeStruct { val: 1 });
        if let (Some(s), Some(t)) = (maybe_s, maybe_t) {
            println!("{:?}", s);
            println!("{:?}", t);
            maybe_s = Some(s);
            maybe_t = Some(t);
        }
        println!("{:?}", maybe_s);
        println!("{:?}", maybe_t);
    }
}
