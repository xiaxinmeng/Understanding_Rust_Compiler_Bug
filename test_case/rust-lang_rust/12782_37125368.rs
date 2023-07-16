 rust
macro_rules! foo {
    ( $( $outer( $e: expr ),* );* ) => {
        (#($outer), 
         #($e))
    }
}
