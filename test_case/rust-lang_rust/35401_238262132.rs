 rust
    for line in rendered_buffer {
        for part in line {
            dst.apply_style(lvl.clone(), part.style)?;
            write!(dst, "{}", part.text)?;
            dst.reset_attrs()?;
        }
        write!(dst, "\n")?;
    }
