

        self.collection.find(None, None)
            .then(|mut res|  async  {
            let mut cursor = res.unwrap();
            let mut users: Vec<UserSchema> = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                        let trader: UserSchema = bson::from_bson(bson::Bson::Document(document)).unwrap();
                        users.push(trader);
                    }
                    Err(e) => return Err(e),
                }
            };
                Ok(current_traders)
        });
