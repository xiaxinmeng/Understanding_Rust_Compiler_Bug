rust
macro_rules! foo {
    ($e:expr) => {
        'a: loop {
            break 'a $e;
        }  
    };
}

'a: loop { 
    foo!(break 'a);
