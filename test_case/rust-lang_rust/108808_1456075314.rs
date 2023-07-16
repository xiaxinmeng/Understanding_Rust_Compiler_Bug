
pub fn main() {
    let mut stats = TestStats::default();
    
    let mut generator = Box::pin(move || {
        // println!("{:?}", stats);
        stats.total_items += 1;
        // yield 2;
        2
    });
    
    
    // Pin::new(&mut generator).resume(());
    generator();
    
}
