rust
        // let objects = futures::future::try_join_all(futs)
        //     .await?
        //     .into_iter()
        //     .collect::<HashMap<Self::Address, C>>();

        let f: Pin<
            Box<
                dyn Future<Output = anyhow::Result<Vec<(<Self as StorageProvider>::Address, C)>>>
                    + Send,
            >,
        > = Box::pin(futures::future::try_join_all(futs));

        f.await;
