
2019-07-30T12:40:57.4149098Z   |   ^
2019-07-30T12:40:57.4149321Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-07-30T12:40:57.4149530Z 1 | ``'
2019-07-30T12:40:57.4149565Z   |   ^
2019-07-30T12:40:57.4149587Z 
2019-07-30T12:40:57.4149631Z warning: could not parse code block as Rust code
---
2019-07-30T12:40:57.4151543Z 
2019-07-30T12:40:57.4151580Z warning: doc comment contains an invalid Rust code block
2019-07-30T12:40:57.4151789Z   --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:63:1
2019-07-30T12:40:57.4151829Z    |
2019-07-30T12:40:57.4151874Z LL | / #[doc = "