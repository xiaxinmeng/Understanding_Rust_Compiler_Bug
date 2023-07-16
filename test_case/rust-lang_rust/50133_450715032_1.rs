console
error[E0119]: conflicting implementations of trait `std::convert::TryFrom<_>` for type `<mod>::DataSet`:
  --> src/<file>.rs:80:1
   |
80 | / impl<P> TryFrom<P> for DataSet
81 | |     where P: AsRef<OsStr>
82 | | {
83 | |     type Error = io::Error;
...  |
89 | |     }
90 | | }
   | |_^
   |
   = note: conflicting implementation in crate `core`:
           - impl<T, U> std::convert::TryFrom<U> for T
             where T: std::convert::From<U>;
