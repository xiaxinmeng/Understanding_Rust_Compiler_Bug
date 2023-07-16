rust
let chars = (0..200).map(|i| {
    let pos = (
        CW * ((i % 50) as f32 + 0.5f32),
        500f32 - CH * ((i / 50) as f32 + 0.5f32),
    )
    //...
}).collect();
