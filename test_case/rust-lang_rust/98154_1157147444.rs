plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0034]: multiple applicable items in scope
    --> library/std/src/io/util/tests.rs:133:18
     |
133  |     assert_eq!(n.by_ref().write(&[0; 1024]).unwrap(), 1024);
     |                  ^^^^^^ multiple `by_ref` found
     |
note: candidate #1 is defined in an impl of the trait `io::Write` for the type `util::Null`
     |
1689 | /     fn by_ref(&mut self) -> &mut Self
1690 | |     where
1691 | |         Self: Sized,
1691 | |         Self: Sized,
     | |____________________^
note: candidate #2 is defined in an impl of the trait `io::Read` for the type `util::Null`
     |
872  | /     fn by_ref(&mut self) -> &mut Self
873  | |     where
874  | |         Self: Sized,
874  | |         Self: Sized,
     | |____________________^
note: candidate #3 is defined in the trait `core::iter::Iterator`
     |
1670 | /     fn by_ref(&mut self) -> &mut Self
1671 | |     where
1672 | |         Self: Sized,
1672 | |         Self: Sized,
     | |____________________^
help: disambiguate the associated function for candidate #1
     |
133  |     assert_eq!(io::Write::by_ref(&mut n).write(&[0; 1024]).unwrap(), 1024);
help: disambiguate the associated function for candidate #2
     |
     |
133  |     assert_eq!(io::Read::by_ref(&mut n).write(&[0; 1024]).unwrap(), 1024);
help: disambiguate the associated function for candidate #3
     |
     |
133  |     assert_eq!(core::iter::Iterator::by_ref(n).write(&[0; 1024]).unwrap(), 1024);

error[E0034]: multiple applicable items in scope
    --> library/std/src/io/util/tests.rs:142:18
     |
     |
142  |     assert_eq!(n.by_ref().read(&mut [0; 1024]).unwrap(), 0);
     |                  ^^^^^^ multiple `by_ref` found
     |
note: candidate #1 is defined in an impl of the trait `io::Write` for the type `util::Null`
     |
1689 | /     fn by_ref(&mut self) -> &mut Self
1690 | |     where
1691 | |         Self: Sized,
1691 | |         Self: Sized,
     | |____________________^
note: candidate #2 is defined in an impl of the trait `io::Read` for the type `util::Null`
     |
872  | /     fn by_ref(&mut self) -> &mut Self
873  | |     where
874  | |         Self: Sized,
874  | |         Self: Sized,
     | |____________________^
note: candidate #3 is defined in the trait `core::iter::Iterator`
     |
1670 | /     fn by_ref(&mut self) -> &mut Self
1671 | |     where
1672 | |         Self: Sized,
1672 | |         Self: Sized,
     | |____________________^
help: disambiguate the associated function for candidate #1
     |
142  |     assert_eq!(io::Write::by_ref(&mut n).read(&mut [0; 1024]).unwrap(), 0);
help: disambiguate the associated function for candidate #2
     |
     |
142  |     assert_eq!(io::Read::by_ref(&mut n).read(&mut [0; 1024]).unwrap(), 0);
help: disambiguate the associated function for candidate #3
     |
     |
142  |     assert_eq!(core::iter::Iterator::by_ref(n).read(&mut [0; 1024]).unwrap(), 0);

error[E0034]: multiple applicable items in scope
    --> library/std/src/io/util/tests.rs:164:7
     |
     |
164  |     n.by_ref().read_buf(&mut buf).unwrap();
     |       ^^^^^^ multiple `by_ref` found
     |
note: candidate #1 is defined in an impl of the trait `io::Write` for the type `util::Null`
     |
1689 | /     fn by_ref(&mut self) -> &mut Self
1690 | |     where
1691 | |         Self: Sized,
1691 | |         Self: Sized,
     | |____________________^
note: candidate #2 is defined in an impl of the trait `io::Read` for the type `util::Null`
     |
872  | /     fn by_ref(&mut self) -> &mut Self
873  | |     where
874  | |         Self: Sized,
874  | |         Self: Sized,
     | |____________________^
note: candidate #3 is defined in the trait `core::iter::Iterator`
     |
1670 | /     fn by_ref(&mut self) -> &mut Self
1671 | |     where
1672 | |         Self: Sized,
1672 | |         Self: Sized,
     | |____________________^
help: disambiguate the associated function for candidate #1
     |
164  |     io::Write::by_ref(&mut n).read_buf(&mut buf).unwrap();
help: disambiguate the associated function for candidate #2
     |
     |
164  |     io::Read::by_ref(&mut n).read_buf(&mut buf).unwrap();
help: disambiguate the associated function for candidate #3
     |
     |
164  |     core::iter::Iterator::by_ref(n).read_buf(&mut buf).unwrap();

For more information about this error, try `rustc --explain E0034`.
error: could not compile `std` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
