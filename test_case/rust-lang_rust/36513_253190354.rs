 rust
error[E0277]: the trait bound `mould_auth::AuthService<checker::Checker<r2d2::Pool<r2d2_postgres::PostgresConnectionManager>>, session::UserRole, dba::Error>: mould::service::Service<session::UserSession>` is not satisfied
  --> src\main.rs:63:11
   |
63 |     suite.register("auth-service", auth_service);
   |           ^^^^^^^^ trait `mould_auth::AuthService<checker::Checker<r2d2::Pool<r2d2_postgres::PostgresConnectionManager>>, session::UserRole, dba::Error>: mould::service::Service<session::UserSession>` not satisfied

error[E0277]: the trait bound `mould_auth::TokenService<checker::Checker<r2d2::Pool<r2d2_postgres::PostgresConnectionManager>>, session::UserRole, dba::Error>: mould::service::Service<session::UserSession>` is not satisfied
  --> src\main.rs:68:11
   |
68 |     suite.register("token-service", token_service);
   |           ^^^^^^^^ trait `mould_auth::TokenService<checker::Checker<r2d2::Pool<r2d2_postgres::PostgresConnectionManager>>, session::UserRole, dba::Error>: mould::service::Service<session::UserSession>` not satisfied

error: aborting due to 2 previous errors
