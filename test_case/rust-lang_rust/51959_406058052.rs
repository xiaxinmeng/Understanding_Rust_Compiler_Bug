plain
[00:40:58] ....................................................................................................
[00:41:01] ....................................................................................................
[00:41:04] ....................................................................................................
[00:41:06] .......................................i............................................................
[00:41:08] ...............................................F....................................................
[00:41:13] ....................................................................................................
[00:41:16] ....................................................................................................
[00:41:19] ....................................................................................................
[00:41:21] ....................................................................................................
---
[00:41:43] 
[00:41:43] + error[E0308]: mismatched types
[00:41:43] +   --> $DIR/issue-20831-debruijn.rs:38:5
[00:41:43] +    |
[00:41:43] + LL | /     fn subscribe(&mut self, t : Box<Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
[00:41:43] + LL | |         // Not obvious, but there is an implicit lifetime here -------^
[00:41:43] + LL | |         //~^^ ERROR cannot infer
[00:41:43] + LL | |         //
[00:41:43] + ...  |
[00:41:43] + LL | |         self.sub = t;
[00:41:43] + LL | |     }
[00:41:43] +    | |_____^ lifetime mismatch
[00:41:43] +    = note: expected type `'a`
[00:41:43] +               found type ``
[00:41:43] +               found type ``
[00:41:43] + note: the anonymous lifetime #2 defined on the method body at 38:5...
[00:41:43] +   --> $DIR/issue-20831-debruijn.rs:38:5
[00:41:43] +    |
[00:41:43] + LL | /     fn subscribe(&mut self, t : Box<Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
[00:41:43] + LL | |         // Not obvious, but there is an implicit lifetime here -------^
[00:41:43] + LL | |         //~^^ ERROR cannot infer
[00:41:43] + LL | |         //
[00:41:43] + ...  |
[00:41:43] + LL | |         self.sub = t;
[00:41:43] + LL | |     }
[00:41:43] +    | |_____^
[00:41:43] + note: ...does not necessarily outlive the lifetime 'a as defined on the impl at 36:6
[00:41:43] +   --> $DIR/issue-20831-debruijn.rs:36:6
[00:41:43] +    |
[00:41:43] + LL | impl<'a> Publisher<'a> for MyStruct<'a> {
[00:41:43] + 
[00:41:43] + error[E0308]: mismatched types
[00:41:43] +   --> $DIR/issue-20831-debruijn.rs:38:5
[00:41:43] +    |
[00:41:43] +    |
[00:41:43] + LL | /     fn subscribe(&mut self, t : Box<Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
[00:41:43] + LL | |         // Not obvious, but there is an implicit lifetime here -------^
[00:41:43] + LL | |         //~^^ ERROR cannot infer
[00:41:43] + LL | |         //
[00:41:43] + ...  |
[00:41:43] + LL | |         self.sub = t;
[00:41:43] + LL | |     }
[00:41:43] +    | |_____^ lifetime mismatch
[00:41:43] +    = note: expected type `'a`
[00:41:43] +               found type ``
[00:41:43] +               found type ``
[00:41:43] + note: the lifetime 'a as defined on the impl at 36:6...
[00:41:43] +   --> $DIR/issue-20831-debruijn.rs:36:6
[00:41:43] +    |
[00:41:43] + LL | impl<'a> Publisher<'a> for MyStruct<'a> {
[00:41:43] +    |      ^^
[00:41:43] + note: ...does not necessarily outlive the anonymous lifetime #2 defined on the method body at 38:5
[00:41:43] +   --> $DIR/issue-20831-debruijn.rs:38:5
[00:41:43] +    |
[00:41:43] + LL | /     fn subscribe(&mut self, t : Box<Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
[00:41:43] + LL | |         // Not obvious, but there is an implicit lifetime here -------^
[00:41:43] + LL | |         //~^^ ERROR cannot infer
[00:41:43] + LL | |         //
[00:41:43] + ...  |
[00:41:43] + LL | |         self.sub = t;
[00:41:43] + LL | |     }
L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20831-debruijn/auxiliary" "-A" "unused"
[00:41:43] ------------------------------------------
[00:41:43] 
[00:41:43] ------------------------------------------
[00:41:43] stderr:
[00:41:43] stderr:
[00:41:43] ------------------------------------------
[00:41:43] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n