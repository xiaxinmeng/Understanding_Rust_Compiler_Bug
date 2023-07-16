
warning: unnecessary braces around block return value
   --> src/models/user.rs:137:63
    |
137 |     fn ticket(&self, context : &SharedContext) -> Option<Ticket> { self.get_ticket(context) }
    |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
