rust
#[test]
fn arc_new_cyclic_deferred_upgrades_succeed() {
    let mut foo = None;
    Arc::new_cyclic(|weak_a| {
        foo = Some(weak_a.clone());

        (1,)
    });
    assert_eq!(*foo.unwrap().upgrade().unwrap(), (1,));
}
