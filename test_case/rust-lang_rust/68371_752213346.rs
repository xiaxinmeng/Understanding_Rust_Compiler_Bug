
    font.layout(todo, *scale, point)
        .enumerate()
        .scan(0., |row_offset, (index, item)| {
            let item_offset = item
                .pixel_bounding_box()
                .map_or_else(|| item.position().x, |x| x.max.x as f32);
            if index == 0 {
                Some(Some(0))
            } else if item_offset - *row_offset <= (WIDTH as f32) {
                Some(None)
            } else {
                *row_offset = *row_offset + item_offset;
                Some(Some(index))
            }
        })
        .flatten()
        .chain(once(todo.len()))
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|indices| &todo[indices[0]..indices[1]])
        .collect::<Vec<&'a str>>()
