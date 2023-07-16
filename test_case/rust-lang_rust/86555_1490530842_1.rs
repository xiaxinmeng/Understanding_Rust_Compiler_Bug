rust
fn main(param: Parameters) -> Handover {
    let serial = Serial::new(param.uart0, /* ... */);
    // ....
    Handover { serial, ..param }
    // instead of: Handover { serial, periph_1: param.periph_1, periph_2: param.periph_2, periph_3: param.periph_3, /* ... */ }
}
