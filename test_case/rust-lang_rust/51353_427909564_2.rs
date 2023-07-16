
    let page_stream : IterOk<PageIterator,()> = iter_ok::<PageIterator,()>(pages);
    let task_stream = page_stream.map(|page| {
        println!("got a page {:?}", page);
//        async {
//            await!(do_page_work(&database, page))
//        };
        Ok(())
    })
    .buffer_unordered(1)
    .for_each(|_| {
        println!("for each!");
        Ok(())
    });
    await!(task_stream);
