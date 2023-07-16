
use uuid;

macro_rules! base_methods {
    () => {
        fn id(&self) -> &uuid::Uuid {
            &self.base.id
        }
    };
}
