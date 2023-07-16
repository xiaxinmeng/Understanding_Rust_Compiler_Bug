rust
fn choose_randomly<G, Item>(
    game: &mut GameState,
    generator: G,
) -> Option<Item>
where
    for<'any> G: FnOnce<(&'any GameState,)>,
    for<'any> <G as FnOnce<(&'any GameState,)>>::Output: Iterator<Item = Item> + 'any,
