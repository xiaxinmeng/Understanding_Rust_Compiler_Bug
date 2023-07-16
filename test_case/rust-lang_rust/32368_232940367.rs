
    if exp < 0 {
        parts[n] = Part::Copy(if upper { b"E-" } else { b"e-" });
        parts[n + 1] = Part::Num(-exp as u16);
    } else {
        parts[n] = Part::Copy(if upper { b"E" } else { b"e" });
        parts[n + 1] = Part::Num(exp as u16);
    }
    &parts[..n + 2]
