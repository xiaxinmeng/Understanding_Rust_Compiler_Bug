rust
#[async_trait]
pub trait TestOperation: Debug {
    async fn execute_on_collection(
        &self,
        collection: &Collection<Document>,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<Bson>>;
    async fn execute_on_database(
        &self,
        database: &Database,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<Bson>>;
    async fn execute_on_client(&self, client: &TestClient) -> Result<Option<Bson>>;
    async fn execute_on_session(&self, session: &mut ClientSession) -> Result<Option<Bson>>;
}
