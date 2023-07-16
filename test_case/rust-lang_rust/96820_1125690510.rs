
struct RefCell<T: ?Sized> {
    borrow_flag: core::cell::Cell<usize>,
    cell: core::cell::UnsafeCell<T>,
}

struct RefStd<'a, T> {
    cell: &'a RefCell<T>,
}

struct Ref<'a, T> {
    cell: &'a core::cell::Cell<usize>,
    value: &'a T,
}

struct RefNew<'a, T> {
    value: *const T,
    cell: &'a core::cell::Cell<usize>,
}

// has noalias
#[no_mangle] fn foo(rc: &RefCell<i32>, r: Ref<'_, i32>) {}
// does not have noalias
#[no_mangle] fn bar(rc: &RefCell<i32>, r: RefStd<'_, i32>) {}
// does not have noalias
#[no_mangle] fn baz(rc: &RefCell<i32>, r: RefNew<'_, i32>) {}
