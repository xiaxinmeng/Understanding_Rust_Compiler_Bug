 rust
macro_rules! dump(                                                   
    ($($a:expr),*) => {                                              
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }                                                                
)                                                                    
