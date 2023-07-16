rust
macro_rules! database_table_doc {
    () => {
        ""
    };
}

#[doc = database_table_doc!()]
#[derive(Debug)]
struct Image {
    #[cfg(test)]
    _f: (),
}

fn main() {}
