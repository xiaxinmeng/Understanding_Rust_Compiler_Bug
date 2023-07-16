
use core::fmt::Debug;
use futures::future::BoxFuture;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = InMemoryStorage;
    run(storage).await;
    Ok(())
}

pub(crate) struct InMemoryStorage;

struct User {
    name: String,
}

impl<'a> StorageRequest<InMemoryStorage> for SaveUser<'a> {
    fn execute(
        &self,
        storage: &InMemoryStorage,
    ) -> BoxFuture<
        Result<<Self as StorageRequestReturnType>::Output, <InMemoryStorage as Storage>::Error>,
    > {
        todo!()
    }
}

trait Storage {
    type Error;
}

impl Storage for InMemoryStorage {
    type Error = String;
}

trait StorageRequestReturnType {
    type Output;
}

trait StorageRequest<S: Storage>: StorageRequestReturnType {
    fn execute(
        &self,
        storage: &S,
    ) -> BoxFuture<Result<<Self as StorageRequestReturnType>::Output, <S as Storage>::Error>>;
}

struct SaveUser<'a> {
    name: &'a str,
}

impl<'a> StorageRequestReturnType for SaveUser<'a> {
    type Output = ();
}

impl User {
    async fn new<S>(storage: &S) -> User
    where
        S: Storage,
    {
        User {
            name: "Joe".to_owned(),
        }
    }

    async fn save<S>(mut self, storage: &S)
    where
        S: Storage,
        for<'a> SaveUser<'a>: StorageRequest<S>,
        <S as Storage>::Error: Debug,
    {
        SaveUser { name: &self.name }
            .execute(storage)
            .await
            .unwrap();
    }
}

async fn run<S>(storage: S)
where
    S: Storage,
    <S as Storage>::Error: Debug,
    for<'a> SaveUser<'a>: StorageRequest<S>,
{
    let user = User::new(&storage).await;
    user.save(&storage).await;
}

