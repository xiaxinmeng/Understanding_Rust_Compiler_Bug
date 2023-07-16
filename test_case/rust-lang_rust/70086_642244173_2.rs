rust
wants_tuple(a.iter().zip(b).next()); //stable
wants_tuple(a.zip(b)); // #![feature(option_zip)]
wants_tuple(try { (a? * b?) }); // #![feature(try_blocks)]
