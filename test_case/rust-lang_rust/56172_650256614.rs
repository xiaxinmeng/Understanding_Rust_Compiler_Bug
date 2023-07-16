rust
#[inline(never)]
pub fn f(clip: Option<&bool>) {
    let item = SpecificDisplayItem::PopStackingContext;
    clip.unwrap();
    do_item(&DI {
            item,
    });
   do_item(&DI {
            item,
    });
}}
