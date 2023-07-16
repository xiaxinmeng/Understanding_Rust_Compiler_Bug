
    let mut ret: Matrix3x4<f64> = Matrix3x4::zeros();
    rotation_t
        .column_iter()
        .zip(ret.column_iter_mut())
        .for_each(|(og, mut new)| {
            new.iter_mut()
                .zip(og.iter())
                .for_each(|(new, og)| *new = *og)
        });
    let mut last_col = ret.column_mut(3);
    last_col
        .iter_mut()
        .zip(translate.iter())
        .for_each(|(ptr, v)| *ptr = *v);
