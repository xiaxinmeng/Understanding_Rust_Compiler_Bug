
std::rand::os::imp::OsRng::new () at src/libstd/macros.rs:321
321     ($e:expr) => (match $e { Ok(e) => e, Err(e) => return Err(e) })
(gdb) print $e
$3 = void
(gdb) print e
$4 = {kind = {{RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {
      RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {
      RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {
      RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {
      RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {
      RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput}, {RUST$ENUM$DISR = InvalidInput, 
      22}, {RUST$ENUM$DISR = InvalidInput}}, desc = {data_ptr = 0x7ffff5662af0 <str26560> "couldn't open file; path=; mode=; access=", 
    length = 18}, detail = {{RUST$ENUM$DISR = Some}, {RUST$ENUM$DISR = Some, {vec = {
          ptr = 0x7ffff4425000 "invalid argument; path=/dev/urandom; mode=open; access=read", '\245' <repeats 69 times>, len = 59, 
          cap = 128}}}}}
