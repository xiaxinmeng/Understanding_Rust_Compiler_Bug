rust
if is_uninhabited {
    mk0(); // eval into a temporary for side effects and/or divergence
} else {
    ptr::write(..., mk0());
}
if is_uninhabited {
    mk1(); // eval into a temporary for side effects and/or divergence
} else {
    ptr::write(..., mk1());
}
// ... and so on for all other fields
