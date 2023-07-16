 Rust
let multi = if let Direction::Right = direction { 1.0 } else { -1.0 };
let mut half_prev_width = 0;
for element in elements.into_iter() {
    let element = element.opacity(props.opacity * element.props.opacity);
    let half_width = width_of(&element) as f64 / 2.0;
    draw_element(element, matrix, g, draw_state);
    let x_trans = half_width + half_prev_width;
    let Transform2D(new_matrix) = Transform2D(matrix)
        .multiply(transform_2d::translation(x_trans * multi, 0.0));
    matrix = new_matrix;
    half_prev_width = half_width;
}

