rust
let _: &dyn Send = &async {
    let collect = [()].iter().map(|it| None::<()>).flatten().collect::<Vec<_>>();
    async {}.await;
};
