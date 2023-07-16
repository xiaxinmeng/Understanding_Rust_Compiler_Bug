rust
impl<T: Ord + Clone> RangeInclusive<T> {
    fn clamp(&self, mut x: T) -> T {
        if x < self.start { x.clone_from(&self.start); }
        else if x > self.end { x.clone_from(&self.end); }
        x
    } 
} 

    assert_eq!((1..=10).clamp(11), 10);

    let strings = String::from("aa")..=String::from("b");
    assert_eq!(strings.clamp(String::from("a")), "aa");
    assert_eq!(strings.clamp(String::from("aaa")), "aaa");
