
use std::cell::RefCell;

thread_local! {
    static TL: RefCell<Option<Box<dyn Fn() -> ToString>>> = RefCell::new(None);
}

pub fn crash(callback: &'static Fn() -> ToString) {
    TL.with(|cc| {
        *cc.borrow_mut() = Some(Box::new(callback));
    });
}
