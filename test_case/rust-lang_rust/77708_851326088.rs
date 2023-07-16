rust
#![feature(const_generics, const_evaluatable_checked)]

trait Delegates<T> {}

struct FileCap<const Op: bool> {}

fn writes_to_path<C>(cap: &C)
where
    C: Delegates<FileCap<{ false }>>,
{
    writes_to_specific_path(&cap);
}

fn writes_to_specific_path<C>(cap: &C)
where
    C: Delegates<FileCap<{ false }>>,
{
}

fn main() {}
