
#[cfg(doctest)]
pub mod check_doc_test {
    pub fn check_doc_macro(){
        doc_comment::doctest!("../README.md");
    }


    #[cfg(doctest)]
    pub fn bar ()
    {
    println!("hello");
    }
}
