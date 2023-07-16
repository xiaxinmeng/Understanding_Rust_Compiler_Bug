rust
let _: &dyn Send = &async {
    let _it = [()].iter().map(|it| None::<()>).flatten();
    async {}.await;
};
