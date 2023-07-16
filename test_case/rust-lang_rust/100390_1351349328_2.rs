rust
    quad_validity.x *= f32_bool(
        all_above_equal(bilinear_at_prev.px0(), 0)
            && all_below(bilinear_at_prev.px0(), out_tex_size),
    );
    quad_validity.y *= f32_bool(
        all_above_equal(bilinear_at_prev.px1(), 0)
            && all_below(bilinear_at_prev.px1(), out_tex_size),
    );
