
#[test]
fn any_referenced() {
    let (a, b, c) = (&5i32 as &Any, &TEST as &Any, &Test as &Any);
    assert!(a.is::<i32>());
    assert!(!b.is::<i32>());
    assert!(!c.is::<i32>());
    assert!(!a.is::<&'static str>());
    assert!(b.is::<&'static str>());
    assert!(!c.is::<&'static str>());
    assert!(!a.is::<Test>());
    assert!(!b.is::<Test>());
    assert!(c.is::<Test>());
}
