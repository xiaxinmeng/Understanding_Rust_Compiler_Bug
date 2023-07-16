
#[get("/threads/{thread_id}/")]
pub async fn view_thread(
    path: web::Path<(i32,)>,
    data: web::Data<MainData<'static>>,
) -> Result<HttpResponse, Error> {
    use crate::orm::threads;
    use futures::try_join;

    let thread = Thread::find_by_id(path.into_inner().0)
        .one(&data.pool)
        .await
        .map_err(|_| error::ErrorInternalServerError("Could not look up thread."))?
        .ok_or_else(|| error::ErrorNotFound("Thread not found."))?;

    // Update thread to include views.
    let tfuture = threads::Entity::update_many()
        .col_expr(
            threads::Column::ViewCount,
            Expr::value(thread.view_count + 1),
        )
        .filter(threads::Column::Id.eq(thread.id))
        .exec(&data.pool)
        //.await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    // Load posts, their ugc associations, and their living revision.
    let pfuture = Post::find()
        .find_also_linked(super::orm::posts::PostToUgcRevision)
        .filter(super::orm::posts::Column::ThreadId.eq(thread.id))
        .all(&data.pool)
        //.await
        .map_err(|_| error::ErrorInternalServerError("Could not find posts for this thread."))?;

    // Multi-thread drifting!
    let (presults, _) =
        try_join!(pfuture, tfuture).map_err(|err| error::ErrorInternalServerError(err))?;

    let mut posts = Vec::new();
    for (p, u) in &presults {
        posts.push(PostForTemplate::from_orm(&p, &u));
    }

    Ok(HttpResponse::Ok().body(ThreadTemplate { thread, posts }.render().unwrap()))
}
