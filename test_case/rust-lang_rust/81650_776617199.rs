rust
fn play_with<'a, T: 'a + Animal + Send>(scope: &Scope<'a>, mut animal: T) {
    scope.spawn(move |_| {
        animal.sleep();
    });
}
