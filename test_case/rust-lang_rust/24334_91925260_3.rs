
main.rs:2:5: 2:17 error: use of unstable library feature 'mycrate_internals'
main.rs:2 use mycrate::bar;
              ^~~~~~~~~~~~
main.rs:2:17: 2:17 help: add #![feature(mycrate_internals)] to the crate attributes to enable
main.rs:2:5: 2:17 warning: unused import, #[warn(unused_imports)] on by default
main.rs:2 use mycrate::bar;
              ^~~~~~~~~~~~
