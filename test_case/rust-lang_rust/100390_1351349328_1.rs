rust
    let vel_diff = f32_bool(
        ((vel_now - vel_prev.xy()) / Vec2::ONE.max(abs_vec2(vel_now + vel_prev.xy()))).length()
            > 0.2,
    );
