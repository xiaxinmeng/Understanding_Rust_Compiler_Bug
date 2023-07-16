
Ok(val) if val.parse::<f32>().is_ok() => val.parse::<usize>().unwrap(),
