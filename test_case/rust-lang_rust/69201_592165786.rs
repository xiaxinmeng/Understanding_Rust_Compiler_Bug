rust
let x = {
    #[cfg(foo)] { 4 }
    #[cfg(not(foo))] { 5 }
};
