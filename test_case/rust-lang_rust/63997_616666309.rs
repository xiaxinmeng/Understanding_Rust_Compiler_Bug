rust
#[repr(transparent)]
struct Wrap<T>(T);

extern "C" fn my_fn() {}

const FN: Wrap<extern "C" fn()> = Wrap(my_fn);

struct Struct {
    fnptr: Wrap<extern "C" fn()>,
}

const fn still_const() -> Struct {
    Struct {
        fnptr: FN,
    }
}
