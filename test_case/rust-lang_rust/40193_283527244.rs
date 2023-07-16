rust
#[test]
fn test_start_end() {
    // path() statically returns Option<VecDeque<(i32, i32)>> 
    let p = path((0,0), (0,0)).unwrap();
    assert_eq!(p, vec![(0, 0)].into_iter().collect());
}
