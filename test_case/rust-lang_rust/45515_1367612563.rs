
error[E0507]: cannot move out of dereference of `DerefContainer<UserData>`
  --> src/main.rs:5:28
   |
5  |     let _new_user_result = user_data.into_new_user(&conn);
   |                            ^^^^^^^^^^--------------------
   |                            |         |
   |                            |         value moved due to this method call
   |                            move occurs because value has type `UserData`, which does not implement the `Copy` trait
   |
note: `UserData::into_new_user` takes ownership of the receiver `self`, which moves value
  --> src/main.rs:67:22
   |
67 |     fn into_new_user(self, _conn: &Conn) -> Result<NewUser, Status<Json<Value>>> {
   |                      ^^^^
