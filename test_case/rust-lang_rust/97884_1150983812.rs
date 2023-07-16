
  |   async fn main() -> Result<(), rocket::Error> {
    |  ______________________________________________^
131 | | }
    | |_^ future created by async block is not `Send` + `Sync`
