
error: byte constant must be ASCII. Use a \xHH escape for a non-ASCII byte
  --> /home/eric/Proj/rust-beta/src/test/ui/attributes/key-value-non-ascii.rs:3:19
   |
LL | #[rustc_dummy = b"ﬃ.rs"] //~ ERROR byte constant must be ASCII
   |                   ^

thread 'rustc' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', compiler/rustc_span/src/lib.rs:1489:17
