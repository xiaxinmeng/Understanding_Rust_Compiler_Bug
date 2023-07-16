
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    tokio::run_async(do_couchie_stuff(args));
}

async fn do_couchie_stuff(args: Args) {
    let database = Arc::new(Database::new(Client::new(), "depot-orders", &args.arg_COUCHDB_URI[..] /* "http://127.0.0.1:8001/api/v1/namespaces/staging/services/couchdb/proxy" */));
    let view = ViewClient::new(database.clone(), "QuotedWare", "by_all_created_at_date");

    let page_size = args.flag_limit;
    let total_pages = await!(view.total_pages(page_size));
    let pages = PageIterator::new(&view, total_pages, page_size);

    let page_stream : IterOk<PageIterator,()> = iter_ok::<PageIterator,()>(pages);
    let task_stream = page_stream.map(|page| {
        println!("got a page {:?}", page);
        async {
            await!(do_page_work(&database, page))
        };
        Ok(())
    })
    .buffer_unordered(1)
    .for_each(|_| {
        println!("for each!");
        Ok(())
    });
    await!(task_stream);
}

async fn do_page_work<'a>(database: &'a Arc<Database<'a>>, page: Page) -> Result<(),()> {
        println!("uri: {}", page.uri);

        let response = await!({
            database.client.get(page.uri.parse().unwrap())
                .timeout(Duration::from_secs(10))
            }).unwrap();

        let body = await!(response.into_body().concat2()).unwrap();
        println!("content len {:?}", body.len());

        let view_response = parse_all_together(&body[..])?;

        for row in view_response.rows {
            let quoted_ware : QuotedWare = row.doc;
            println!("qw {} {:?}", quoted_ware.created_at, quoted_ware.quote_group_id);
        }

        Ok(())
}
