rust
        fn convert(objects: Vec<LoadedObject>) -> (Vec<Body>, Vec<Graphic>) {
            objects
                .into_iter()
                .flat_map(|LoadedObject { bodies, color, .. }| {
                    bodies.into_iter()
                        .map(move |body| (body, Graphic { color: color.clone() }))
                })
                .unzip()
        }
