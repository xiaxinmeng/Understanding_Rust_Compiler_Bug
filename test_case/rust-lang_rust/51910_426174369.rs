rust
union Transmuter {
    from: LPCWSTR,
    to: usize,
}
const MY_CONST: usize = unsafe { Transmuter { from: IDC_ARROW }.to };
