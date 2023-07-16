
error: any use of this value will cause an error
  --> src/chess/attacks.rs:70:5
   |
21 | / const ROOK_ATTACK_OFFSETS: [usize; BOARD_SIZE as usize] =
22 | |     generate_table::<ROOK_ATTACKS_COUNT>(&ROOK_ATTACK_DIRECTIONS).2;
   | |____________________________________________________________________-
...
70 |       while scanning_bit < 64 {
   |  _____^
   | |_____|
   | |_____|
   | |_____|
   | |
71 | |         if mask == 0 {
72 | |             break;
73 | |         }
...  |
79 | |         scanning_bit += 1;
80 | |     }
   | |     ^
   | |_____|
   | |_____exceeded interpreter step limit (see `#[const_eval_limit]`)
   | |_____inside `pdep` at src/chess/attacks.rs:70:5
   | |_____inside `generate_table::<102400_usize>` at src/chess/attacks.rs:128:35
   |       inside `ROOK_ATTACK_OFFSETS` at src/chess/attacks.rs:22:5
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
