rust
macro_rules! lint_array { 
    ($( $lint:expr ),*,) => { lint_array!( $( &$lint ),* ) };
    ($( $lint:expr ),*) => {{ 
         static ARRAY: LintArray = &[ $( &$lint ),* ]; 
         ARRAY 
    }} 
} 
