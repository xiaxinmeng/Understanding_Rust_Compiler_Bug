rust
#[harness::test]
async fn select_equals() -> Result<(), Box<dyn Error>> {
    let client = connect().await?;

    assert_eq!(
        test::select()
            .where_eq(test::bar(), 32)
            .with(test::bar())
            .execute(&client)
            .await?
            .collect()
            .await?,
        vec![32]
    );

    Ok(())
}
