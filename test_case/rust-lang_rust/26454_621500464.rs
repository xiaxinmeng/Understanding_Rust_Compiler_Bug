rust
error[E0599]: no method named `address` found for mutable reference `&mut actix_web_actors::ws::WebsocketContext<ECall>` in the current scope
  --> src/main.rs:64:20
   |
64 |     let addr = ctx.address();
   |                    ^^^^^^^ method not found in `&mut actix_web_actors::ws::WebsocketContext<ECall>`
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
2  | use actix::actor::AsyncContext;

error[E0603]: module `actor` is private
  --> src/main.rs:11:12
   |
11 | use actix::actor::AsyncContext;
   |            ^^^^^ this module is private
   |
