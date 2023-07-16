
use bson::{doc};
use bson::oid::ObjectId;
use mongodb::{error::Error, Collection, options::FindOneOptions};

#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

impl UserService {
    pub async fn get_all(&self) -> Result<Vec<UserSchema>, Error> {
        let mut cursor = self.collection.find(None, None).await?; // That's the bad line
        let mut vec: Vec<UserSchema> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user: UserSchema = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    vec.push(user);
                }
                Err(e) => return Err(e.into()),
            }
        };
        Ok(vec)
    }
}
