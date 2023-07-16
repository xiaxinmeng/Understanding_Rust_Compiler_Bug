

if let Some(ref block_record) = cursor
            .map(|document| {
                let document = document.unwrap();
                bson::from_bson(bson::Bson::Document(document)).unwrap()
            })
            .next() {
    let block_record: BlocksRecord = block_record;
    Ok(block_record)
} else {
/...
}
