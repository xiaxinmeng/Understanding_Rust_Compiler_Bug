
$ grep 'try!' **/*.rs | grep -v 'panictry!' | grep -v 'option_try!' | grep -v ' *//'
src/libcore/num/bignum.rs:                try!(write!(f, "{:#x}", self.base[sz-1]));
src/libcore/num/bignum.rs:                    try!(write!(f, "_{:01$x}", v, digitlen));
src/librustc_const_eval/int.rs:                match try!(self.infer(rhs)) {
src/librustc_const_eval/int.rs:                match try!(self.infer(rhs)) {
src/librustc/diagnostics.rs:Another situation in which this occurs is when you attempt to use the `try!`
src/librustc/diagnostics.rs:    let mut f = try!(File::create("foo.txt"));
src/librustc/diagnostics.rs:`try!` returns a `Result<T, E>`, and so the function must. But `main()` has
src/libserialize/json.rs:            try!(write!($enc.writer, "\"{}\"", $e));
src/libserialize/json.rs:            try!(write!($enc.writer, "{}", $e));
src/libserialize/serialize.rs:                    let ret = ($(try!(d.read_tuple_arg({ i+=1; i-1 },
src/libserialize/serialize.rs:                    $(try!(s.emit_tuple_arg({ i+=1; i-1 }, |s| $name.encode(s)));)*
src/libstd/sys/common/backtrace.rs:                                try!(writer.write_all($demangled));
src/libstd/sys/common/backtrace.rs:                                try!(writer.write_all(rest.as_bytes()));
