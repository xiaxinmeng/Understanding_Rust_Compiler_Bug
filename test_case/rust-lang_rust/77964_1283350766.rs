
error[E0271]: type mismatch resolving `<impl std::future::Future<Output = Result<ServiceRequest, actix_web::Error>> as std::future::Future>::Output == Result<ServiceRequest, (actix_web::Error, ServiceRequest)>`
   --> src/main.rs:155:42
    |
155 |     let auth: HttpAuthentication<_, _> = HttpAuthentication::basic(password_check).await;
    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^ expected tuple, found struct `actix_web::Error`
    |
    = note: expected enum `Result<_, (actix_web::Error, ServiceRequest)>`
               found enum `Result<_, actix_web::Error>`
