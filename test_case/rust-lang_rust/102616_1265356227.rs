
    rt.block_on(async {
        if let Err(e) = client.register().await {
            Err(e)
        } else {
            Ok(())
        }
    })?;
