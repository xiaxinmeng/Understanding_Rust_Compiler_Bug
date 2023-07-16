rs
let mut table = table![row![1, 2], row![3, 4], row![5, 6]]

table
    .iter_row_mut()
    .enumerate()
    .for_each(|(i, row)| row.color = if i.is_even() { Color::BLACK } else { Color::DARK_GREY })
    
// as opposed to

table
    .iter_row_mut()
    .enumerate()
    .for_each(|(i, row)| row.color = if i % 2 == 0 { Color::BLACK } else { Color::DARK_GREY })
