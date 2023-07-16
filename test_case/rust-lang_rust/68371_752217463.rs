rust
font.layout(todo, *scale, point)
    .enumerate()
    .map({ let row_offset = 0.0; |(index, item)| {
        let item_offset = item
            .pixel_bounding_box()
            .map_or_else(|| item.position().x, |x| x.max.x as f32);
        if index == 0 {
            Some(0)
        } else if item_offset - row_offset <= (WIDTH as f32) {
            None
        } else {
            row_offset += item_offset;
            Some(index)
        }
    }})
    .flatten()
    .chain(once(todo.len()))
    .collect::<Vec<usize>>()
    .windows(2)
    .map(|indices| &todo[indices[0]..indices[1]])
    .collect::<Vec<&'a str>>()
