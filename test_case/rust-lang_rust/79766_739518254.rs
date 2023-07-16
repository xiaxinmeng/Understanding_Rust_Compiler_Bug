
Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffed9ff700 (LWP 10618)]
0x00007ffff5dc6b21 in rustc_parse::parser::TokenCursor::next ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
(gdb) where
#0  0x00007ffff5dc6b21 in rustc_parse::parser::TokenCursor::next ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#1  0x00007ffff5dd13c6 in rustc_parse::parser::Parser::bump ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#2  0x00007ffff5dc8462 in rustc_parse::parser::Parser::expect_one_of ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#3  0x00007ffff5dc7cc9 in rustc_parse::parser::Parser::expect ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#4  0x00007ffff5d7f059 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_bottom_expr ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#5  0x00007ffff5d79a92 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_prefix_expr::{{closure}} ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#6  0x00007ffff5d78798 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_prefix_expr ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#7  0x00007ffff5d78d52 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_prefix_expr::{{closure}} ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#8  0x00007ffff5d78798 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_prefix_expr ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#9  0x00007ffff5d757a8 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_assoc_expr_with ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#10 0x00007ffff5d75363 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_expr_catch_underscore ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#11 0x00007ffff5dcc794 in rustc_parse::parser::Parser::parse_seq_to_before_tokens ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#12 0x00007ffff5d7f0f8 in rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_bottom_expr ()
