plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0034]: multiple applicable items in scope
    --> library/std/src/io/util/tests.rs:80:18
     |
80   |     assert_eq!(e.by_ref().read(&mut [0; 1024]).unwrap(), 0);
     |                  ^^^^^^ multiple `by_ref` found
     |
note: candidate #1 is defined in an impl of the trait `io::Read` for the type `util::Empty`
     |
872  | /     fn by_ref(&mut self) -> &mut Self
873  | |     where
874  | |         Self: Sized,
874  | |         Self: Sized,
     | |____________________^
note: candidate #2 is defined in an impl of the trait `io::Write` for the type `util::Empty`
     |
1707 | /     fn by_ref(&mut self) -> &mut Self
1708 | |     where
1709 | |         Self: Sized,
1709 | |         Self: Sized,
     | |____________________^
note: candidate #3 is defined in the trait `core::iter::Iterator`
     |
1714 | /     fn by_ref(&mut self) -> &mut Self
1715 | |     where
1716 | |         Self: Sized,
1716 | |         Self: Sized,
     | |____________________^
help: disambiguate the associated function for candidate #1
     |
80   |     assert_eq!(io::Read::by_ref(&mut e).read(&mut [0; 1024]).unwrap(), 0);
help: disambiguate the associated function for candidate #2
     |
     |
80   |     assert_eq!(io::Write::by_ref(&mut e).read(&mut [0; 1024]).unwrap(), 0);
help: disambiguate the associated function for candidate #3
     |
     |
80   |     assert_eq!(core::iter::Iterator::by_ref(e).read(&mut [0; 1024]).unwrap(), 0);

error[E0034]: multiple applicable items in scope
    --> library/std/src/io/util/tests.rs:102:7
     |
     |
102  |     e.by_ref().read_buf(&mut buf).unwrap();
     |       ^^^^^^ multiple `by_ref` found
     |
note: candidate #1 is defined in an impl of the trait `io::Read` for the type `util::Empty`
     |
872  | /     fn by_ref(&mut self) -> &mut Self
873  | |     where
874  | |         Self: Sized,
874  | |         Self: Sized,
     | |____________________^
note: candidate #2 is defined in an impl of the trait `io::Write` for the type `util::Empty`
     |
1707 | /     fn by_ref(&mut self) -> &mut Self
1708 | |     where
1709 | |         Self: Sized,
1709 | |         Self: Sized,
     | |____________________^
note: candidate #3 is defined in the trait `core::iter::Iterator`
     |
1714 | /     fn by_ref(&mut self) -> &mut Self
1715 | |     where
1716 | |         Self: Sized,
1716 | |         Self: Sized,
     | |____________________^
help: disambiguate the associated function for candidate #1
     |
102  |     io::Read::by_ref(&mut e).read_buf(&mut buf).unwrap();
help: disambiguate the associated function for candidate #2
     |
     |
102  |     io::Write::by_ref(&mut e).read_buf(&mut buf).unwrap();
help: disambiguate the associated function for candidate #3
     |
     |
102  |     core::iter::Iterator::by_ref(e).read_buf(&mut buf).unwrap();

For more information about this error, try `rustc --explain E0034`.
error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
