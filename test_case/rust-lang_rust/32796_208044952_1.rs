 rust
let x = (#[cfg(a)] foo(), #[cfg(not(a))] bar()).0;
