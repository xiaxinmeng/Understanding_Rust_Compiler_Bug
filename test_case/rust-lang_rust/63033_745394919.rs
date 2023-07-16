
    async fn foo(
        &self,
        user_id: &Uuid,
        character_id: &Uuid,
        item_id: &Uuid,
        topic: &'static str,
    ) -> Result<(), String>
