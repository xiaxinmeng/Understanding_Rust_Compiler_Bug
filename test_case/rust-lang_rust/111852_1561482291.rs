rust
use futures::future::{BoxFuture, FutureExt};

trait Executor: Send {
    type Executor<'this>: sqlx::PgExecutor<'this>
    where
        Self: 'this;
    fn as_executor(&mut self) -> Self::Executor<'_>;
}

struct Store {}

#[derive(sqlx::FromRow)]
struct Team {}

fn query_teams(user: &str, db: impl Executor, store: Store) -> BoxFuture<Store> {
    async move {
        let query = sqlx::QueryBuilder::new(r#"SELECT "teams".*" FROM "teams"#);
        query
            .build_query_as::<Team>()
            .fetch_all(db.as_executor())
            .await;
        store
    }
    .boxed()
}
