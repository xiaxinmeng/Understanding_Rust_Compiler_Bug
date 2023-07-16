rust
#[derive(Debug, sqlx::FromRow)]
struct Grid {
  id: i64,
  row: f32,
};

struct Target {
  id: i64,
  grid_id: Option<i64>,
  ...
}

/// Just for namespacing
struct GridSql;

impl GridSql {
    pub async fn add_empty<'t>(
        target: &Target,
        mut tx: sqlx::Transaction<'t, Postgres>,
    ) -> sqlx::Result<sqlx::Transaction<'t, Postgres>> {
        let grid_id = sqlx::query_as!(
            Grid,
            r#"
            INSERT INTO grid (
               row
            ) VALUES($1)
            RETURNING
                id, ...
            "#,
            0.0,
        )
        .fetch_one(&mut tx)
        .await?
        .id;
        sqlx::query!(
            "UPDATE target SET grid_id = $1 WHERE id = $2",
            grid_id,
            target.id,
        )
        .execute(&mut tx)
        .await?;
        Ok(tx)
    }
}
