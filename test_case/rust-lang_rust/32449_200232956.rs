 rust
const A: [fn(); 4] = [
    {fn tmp() { unsafe { b = 4 } } tmp},
    {fn tmp() { unsafe { b = 3 } } tmp},
    {fn tmp() { unsafe { b = 2 } } tmp},
    {fn tmp() { unsafe { b = 1 } } tmp},
];
