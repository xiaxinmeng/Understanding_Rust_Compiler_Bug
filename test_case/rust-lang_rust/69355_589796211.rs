
    let outbound = async_stream::stream! {
        let input = CreateWallInput {
            file: String::from("test_ops_client"),
            user: String::from("test_ops_user"),
            wall: Some(WallMsg {
                first_pt: Point3Msg {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                second_pt: Some(Point3Msg {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }),
                width: 1.0,
                height: 1.0,
            }),
        };
        yield input;
    };
