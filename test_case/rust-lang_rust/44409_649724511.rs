rust

fn test<'a, 'b: 'a>(x: &'a mut &'b mut i32) -> &'b mut i32 {
    &mut **x
}

