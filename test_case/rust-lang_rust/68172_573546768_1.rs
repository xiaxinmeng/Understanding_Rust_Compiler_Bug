
//change code  to 'a   ,it will  success,but  
impl<'a> Sesssion<'a> {
    pub fn push_stack(&'a mut self) {
        self.connection = "".to_string();
        self.stack.t = Some(Transaction {
            conn_ref: &mut self.connection,
        });
    }
}
