rust
fn choose_randomly<It>(
    game: &mut GameState,
    generator: impl FnOnce(&GameState) -> It,
) -> Option<It::Item>
where
    It: Iterator,
