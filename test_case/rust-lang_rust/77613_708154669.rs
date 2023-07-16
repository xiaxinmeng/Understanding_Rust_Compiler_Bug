rust
pub fn g(clip: bool) {
    if clip {
        return;
    }

    let item = SpecificDisplayItem::PopStackingContext;
    do_item(&DI {
            item,
    });
}
