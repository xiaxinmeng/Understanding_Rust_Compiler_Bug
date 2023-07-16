rust
#![feature(macro_metavar_expr)]

fn main() {
    macro_rules! mac {
        ( $( [ $( $i:ident )* ] )* ) => {{
            // ***** No loop *****
            
            println!("{}", ${count(i)}); // 5
            println!("{}", ${count(i, 0)}); // 2
            
            // Same as ${count(i)}
            //println!("{}", ${count(i, 1)});
            
            // Fobirdden. Index out of bounds
            //println!("{}", ${count(i, 2)});
            
            // ***** Outer-most loop *****
            
            $(
                println!("{}", ${count(i)}); // 3 and 2
                
                // Same as ${count(i)}
                //println!("{}", ${count(i, 0)});
                
                // Fobirdden. Index out of bounds
                //println!("{}", ${count(i, 1)});
            )*

            // ***** Outer-most and inner-most loops *****
            
            $(
                $(
                    ${ignore(i)}

                    // Forbidden. Can't be placed inside the inner-most repetition
                    //println!("{}", ${count(i)});
                )*
            )*
        }};
    }
    
    mac!([a b c] [d e]);
}
