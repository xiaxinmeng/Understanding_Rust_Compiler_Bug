rust
async fn use_my_struct<'a, 'b: 'a>(val: MyStruct<'a, &'b u8>) {
    use_async(val).await;
}
