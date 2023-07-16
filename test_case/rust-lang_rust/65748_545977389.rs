rust
#![feature(untagged_unions)]

struct NoCopy;

#[derive(Clone, Copy)]
struct IsCopy;

#[derive(Clone, Copy)]
union U {
    a: NoCopy,
    b: IsCopy,
}
