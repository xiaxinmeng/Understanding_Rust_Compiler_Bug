
pub fn dopow(mut exp: u32) -> u64 {
            let mut acc = 1;
            while exp > 0 {
                acc = acc * 2;
                exp -= 1;
            }
            acc
}
