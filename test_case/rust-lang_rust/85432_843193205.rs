rust
macro_rules! with_doc_comment {
    ($comment:expr, $item:item) => {
        #[doc = $comment]
        $item
    };
}

macro_rules! database_table_doc {
    () => {
        ""
    };
}

with_doc_comment! {
    database_table_doc!(),
    #[derive(Debug)]
    struct Image {
        #[cfg(test)]
        _f: (),
    }

}

fn main() {}
