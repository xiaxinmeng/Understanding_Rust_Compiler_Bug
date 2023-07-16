rust
let res: MyResponse = client.get("https://my_api").send().await?.json().await?;
