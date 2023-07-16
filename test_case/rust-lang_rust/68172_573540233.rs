rust
impl<'a> Sesssion<'a> {
    pub fn push_stack<'b>(&'b mut self) { // let's call `self`'s lifetime `b`
        self.connection = "".to_string();
        self.stack.t = Some(Transaction {
            conn_ref: &mut self.connection,
                   // ^ this borrow has a lifetime of `'b` (because it's tied to
                   // `&mut self`), but - at the same time - in your struct's
                   // definition you require `conn_ref` to have a lifetime of `'a`;
                   //
                   // since `'b` doesn't necessarily live as long as `'a` (think:
                   // `Session<'static>` for example), it's an invalid code; in C++
                   // you'd end up with use-after-free at runtime, wheras Rust
                   // explicitly tells you that you got the lifetimes wrong;
                   //
                   // generally I think that your approach is somewhat ill-designed,
                   // but it's hard to say without knowing the context 
        });
    }
}
