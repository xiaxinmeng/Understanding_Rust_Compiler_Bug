
error: any use of this value will cause an error
  --> src/chess/attacks.rs:52:35
   |
14 | / const BISHOP_ATTACK_OFFSETS: [usize; BOARD_SIZE as usize] =
15 | |     generate_table::<BISHOP_ATTACKS_COUNT>(&BISHOP_ATTACK_DIRECTIONS).2;
   | |________________________________________________________________________-
...
52 |               let attacked_square = to_square(column, row);
   |                                     ^^^^^^^^^^^^^^^^^^^^^^
   |                                     |
   |                                     exceeded interpreter step limit (see `#[const_eval_limit]`)
   |                                     inside `generate_attacks` at src/chess/attacks.rs:52:35
   |                                     inside `generate_table::<5248_usize>` at src/chess/attacks.rs:129:61
   |                                     inside `BISHOP_ATTACK_OFFSETS` at src/chess/attacks.rs:15:5
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
