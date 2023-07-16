rust
fn choose_randomly<G, Item>(
    game: &mut GameState,
    generator: G,
) -> Option<Item>
where
    for<'any> G: GeneratesIterator<'any, Item=Item>,
