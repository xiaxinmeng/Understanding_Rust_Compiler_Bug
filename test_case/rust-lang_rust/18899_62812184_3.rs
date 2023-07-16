
match max {
    None => Some((x, x_val)),
    Some((y, y_val)) => {
        unsafe{
            println!("{}, {}",
            *std::mem::transmute::<&B, *const uint>(&x_val),
            *std::mem::transmute::<&B, *const uint>(&y_val));
        }
        if x_val > y_val {
            Some((x, x_val))
        } else {
            Some((y, y_val))
        }
    }
}
