\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-42060.rs","byte_start":514,"byte_end":527,"line_start":13,"line_end":13,"column_start":16,"column_end":29,"is_primary":true,"text":[{"text":"    let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant","highlight_start":16,"highlight_end":29}],"label":"reserved keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0516]: `typeof` is a reserved keyword but unimplemented\n  --> /checg) {}
[01:00:57] +    |                       ^ doesn't have a size known at compile-time
[01:00:57] 6    |
[01:00:57] -    = help: the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`
[01:00:57] +    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::string::ToString + 'static)`
[01:00:57] 8    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:00:57] -    = help: consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound
[01:00:57] 10    = note: all function arguments must have a statically known size
[01:00:57] 12 
[01:00:57] 
[01:00:57] 
[01:00:57] - error[E0277]: the size for values of type `(dyn std::string::ToString + 'static)` cannot be known at compilation time
[01:00:57] -   --> $DIR/issue-42312.rs:18:23
[01:00:57] + error[E0277]: the size for values of type `<Self as std::ops::Deref>::Target` cannot be known at compilation time
[01:00:57] +   --> $DIR/issue-42312.rs:14:29
[01:00:57] 15    |
[01:00:57] - LL | pub fn f(_: ToString) {}
[01:00:57] -    |                       ^ doesn't have a size known at compile-time
[01:00:57] + LL |     fn baz(_: Self::Target) where Self: Deref {}
[01:00:57] +    |                             ^ doesn't have a size known at compile-time
[01:00:57] 18    |
[01:00:57] -    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::string::ToString + 'static)`
[01:00:57] +    = help: the trait `std::markere of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-42312.rs","byte_start":533,"byte_end":533,"line_start":14,"line_end":14,"column_start":29,"column_end":29,"is_primary":true,"text":[{"text":"    fn baz(_: Self::Target) where Self: Deref {}","highlight_start":29,"highlight_end":29}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"all function arguments must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"unsized locals are gated as an unstable feature","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `<Self as std::ops::Deref>::Target` cannot be known at compilation time\n  --> /checkout/src/test/ui/issues/issue-42312.rs:14:29\n   |\nLL |     fn baz(_: Self::Target) where Self: Deref {}\n35.rs:16:20\n   |\nLL |     id(Box::new(|| *v))\n   |                    ^^ cannot move out of borrowed content\n\n"}
[01:00:57] {"message":"closure may outlive the current function, but it borrows `v`, which is owned by the current function","code":{"code":"E0373","explanation":"\nThis error occurs when an attempt is made to use data captured by a closure,\nwhen that data may no longer exist. It's most commonly seen when attempting to\nreturn a closure:\n\n