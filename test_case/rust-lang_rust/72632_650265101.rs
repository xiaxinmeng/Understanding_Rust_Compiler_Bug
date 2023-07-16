rust
#[inline(never)]
pub fn g(clip: Option<&bool>) {
    clip.unwrap();
    let item = SpecificDisplayItem::PopStackingContext;
    do_item(&DI {
            item,
    });
    do_item(&DI {
            item,
    });
}

pub enum SpecificDisplayItem {
    PopStackingContext,
    Other([f64; 22]),
}

struct DI {
    item: SpecificDisplayItem,
}


fn do_item(di: &DI) { unsafe { ext(di) } }
extern {
    fn ext(di: &DI);
}
