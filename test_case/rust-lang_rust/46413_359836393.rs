rust
#![feature(nll)]

pub struct Statement;

pub struct Rows<'stmt>(&'stmt Statement);

impl<'stmt> Drop for Rows<'stmt> {
    fn drop(&mut self) {}
}

impl<'stmt> Iterator for Rows<'stmt> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn get_names() -> Option<String> {
    let stmt = Statement;
    let rows = Rows(&stmt);
    rows.map(|row| row).next()
    // Replacing the previous line with the following works:
    // let x = rows.map(|row| row).next();
    // x
    //
    // Removing the map works too as does removing the Drop impl.
}

fn main() {}
