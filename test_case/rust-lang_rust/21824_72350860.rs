 rust
#[test]
#[should_pass]
fn test_oob() {
    assert_panic!("out of bounds", {        
        vec![][10]
    }); // if the block executed successfully, we panic
}
