rust
pub fn top<Table: diesel::Table + diesel::query_dsl::InternalJoinDsl<_, diesel::query_source::joins::Inner, _>>(table: Table, limit: usize, connection: DbConn) -> RestResult<Vec<TopWineType>> {
    table
        .inner_join(wines::table.inner_join(purchases::table))
        .group_by((producers::id, producers::name))
        .select((
            wines::id,
            wines::name,
            sql::<Integer>("sum(purchases.quantity)"),
            // Should probably be distinct
            sql::<Integer>("count(wines.id)"),
            sql::<Float>("avg(purchases.price)"),
        ))
        .order_by(sql::<Integer>("sum(purchases.quantity) DESC"))
        .limit(limit as i64)
        .load::<TopWineType>(&*connection)
        .map(Json)
        .map_err(VinotecaError::from)
}
