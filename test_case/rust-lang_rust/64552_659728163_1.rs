
pub fn api_admin(db: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "admin")
        .and(warp::get())
        .and(require_admin(db.clone()))
        .and(with_db(db))
        .and_then(api_admin_get)
}

async fn api_admin_get(_admin: User, db: PgPool) -> Result<impl Reply, Infallible> {
    ...

    let tags = try_or_500!({
        use itertools::Itertools;
        Tag::find_all_in_list(
            &mut conn,
            &series_tags
                .iter()
                .map(|st| st.tag_id)
                .unique()
                .collect::<Vec<_>>(),
        )
        .await
    });

    ...

    Ok(...)
}
