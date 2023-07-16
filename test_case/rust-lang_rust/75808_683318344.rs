rust
use actix_web::{web::Data, HttpResponse};
use tokio_postgres::Client;

pub trait HttpServiceFactory {
    fn register(self);
}
pub struct Complete;
impl HttpServiceFactory for Complete {
    fn register(self) {
        pub async fn complete(client: Data<Client>) -> HttpResponse {
            async { create(&client).await }.await;
            HttpResponse::NotFound().finish()
        }
        actix_web::Resource::new("/").to(complete);
    }
}
async fn create(client: &Client) {
    async move {
        get_by_authentication(client).await;
        client.query_one("", &[]).await.unwrap();
    }
    .await
}
async fn get_by_authentication(client: &Client) {
    async move {
        client.query_one("", &[]).await.unwrap();
    }
    .await
}
