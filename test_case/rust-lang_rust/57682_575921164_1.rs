rust
    pub async fn fetch_fortunes(&self) -> Vec<Fortune> {
        self.client
            .query_raw(&self.fortune, iter::empty())
            .await
            .unwrap()
            .map(|row| row.unwrap())
            .map(|row| Fortune {
                id: row.get(0),
                message: Cow::Owned(row.get(1)),
            })
            .collect::<Vec<_>>()
            .await
    }
