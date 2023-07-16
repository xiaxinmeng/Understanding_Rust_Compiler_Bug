rust
let is_phantom_safe = adt_def.struct_variant().fields().iter().any(|field| {
    !(field_type).sty.is_phantom_data() 
});
