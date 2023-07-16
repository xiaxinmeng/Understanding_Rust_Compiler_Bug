
let a = ~7;
unsafe {
    let x : *uint = unsafe::reinterpret_cast(*a);
    let y : *uint = unsafe::reinterpret_cast(a);
    let z : *uint = unsafe::reinterpret_cast(&a);
}
