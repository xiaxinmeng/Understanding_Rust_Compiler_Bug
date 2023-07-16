rust
async fn obstest() -> Result<impl Responder, MyError> {
    let obs_connect = || -> Result<(obws::Version, Vec<obws::responses::scenes::Scene>)>, MyError) {
        async {
            let client = Client::connect("localhost", 4455, Some("1234567890")).await?;
            let version = client.general().version().await?.obs_version;
            let scene_list = client.scenes().list().await?.scenes;
            Ok((version, scene_list))
        }
    }

    if let Ok(version, scene_list) = obs_connect() {
        Ok(HttpResponse::Ok().body(format!("Hello from OBS {:?}\nScenes are: {:?}", version, scene_list)))
    } else {
        Err(MyError { name: "test" })
    }
}
