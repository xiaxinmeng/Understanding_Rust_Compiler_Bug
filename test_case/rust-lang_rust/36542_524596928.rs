rust
error[E0596]: cannot borrow `d` as mutable, as it is not declared as mutable
  --> src\player.rs:20:45
   |
14 | pub fn take_turn(p: &mut Player, m: &mut Vec<Meeple>, d: &mut Vec<Card>, f: &mut Vec<Card>) {
   |                                                       - help: consider changing this to be mutable: `mut d`
...
20 |         Org::State        => state_turn(&mut p, &mut m, &mut d),
   |                                                         ^^^^^^ cannot borrow as mutable
