rust
let variant_layout = place.layout().for_variant(fx, variant_idx); // Remove `for_variant` when zeroing during the prologue
let mut padding_ranges = vec![];
match &variant_layout.fields {
    layout::FieldPlacement::Union(field_count) => {
        padding_ranges.push(
            (0..*field_count).map(|field| variant_layout.field(fx, field).size).max().unwrap_or(Size::ZERO)
            ..
            place.layout().size,
        );
    }
    layout::FieldPlacement::Array { stride, count } => {
        for field in 0..*count as usize {
            padding_ranges.push(
                variant_layout.fields.offset(field) + variant_layout.field(fx, field).size
                ..
                variant_layout.fields.offset(field) + *stride
            );
        }
    }
    layout::FieldPlacement::Arbitrary { .. } => {
        let mut last_field = None;
        for field in variant_layout.fields.index_by_increasing_offset() {
            if let Some(last_field) = last_field {
                padding_ranges.push(
                    variant_layout.fields.offset(last_field) + variant_layout.field(fx, last_field).size
                    ..
                    variant_layout.fields.offset(field)
                );
            }
            last_field = Some(field);
        }
        if let Some(last_field) = last_field {
            padding_ranges.push(
                variant_layout.fields.offset(last_field) + variant_layout.field(fx, last_field).size
                ..
                variant_layout.size
            );
        } else {
            //padding_ranges.push(Size::ZERO..variant_layout.size);
        }
    }
}

if let ty::Adt(adt_def, _) = place.layout().ty.kind {
    //if adt_def.is_struct() {
    //} else {
    //    padding_ranges.clear();
    //}
} else {
    padding_ranges.clear();
}

for padding_range in padding_ranges {
    let addr = place.to_ptr(fx).offset_i64(fx, i64::try_from(padding_range.start.bytes()).unwrap()).get_addr(fx);
    let size = padding_range.end.bytes() - padding_range.start.bytes();
    fx.bcx.emit_small_memset(
        fx.module.target_config(),
        addr,
        0,
        size,
        std::cmp::min(/*greatest_divisible_power_of_two*/(size as i64 & -(size as i64)) as u64, 8u64) as u8, /*FIXME*/
    );
}
