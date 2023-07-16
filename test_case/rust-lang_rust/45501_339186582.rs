
[00:58:51] error[E0599]: no method named `position` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:20:15
[00:58:51] |
[00:58:51] 20 | assert!(b.position(|&v| v == 9) == None);
[00:58:51] | ^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `position` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:21:15
[00:58:51] |
[00:58:51] 21 | assert!(b.position(|&v| v == 5) == Some(3));
[00:58:51] | ^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `position` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:22:15
[00:58:51] |
[00:58:51] 22 | assert!(b.position(|&v| v == 3) == Some(2));
[00:58:51] | ^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `position` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:23:15
[00:58:51] |
[00:58:51] 23 | assert!(b.position(|&v| v == 0) == None);
[00:58:51] | ^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `rposition` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:29:15
[00:58:51] |
[00:58:51] 29 | assert!(b.rposition(|&v| v == 9) == None);
[00:58:51] | ^^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `rposition` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:30:15
[00:58:51] |
[00:58:51] 30 | assert!(b.rposition(|&v| v == 5) == Some(4));
[00:58:51] | ^^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `rposition` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:31:15
[00:58:51] |
[00:58:51] 31 | assert!(b.rposition(|&v| v == 3) == Some(2));
[00:58:51] | ^^^^^^^^^
[00:58:51]
[00:58:51] error[E0599]: no method named `rposition` found for type `[{integer}; 5]` in the current scope
[00:58:51] --> /checkout/src/libcore/../libcore/tests/slice.rs:32:15
[00:58:51] |
[00:58:51] 32 | assert!(b.rposition(|&v| v == 0) == None);
[00:58:51] | ^^^^^^^^^
