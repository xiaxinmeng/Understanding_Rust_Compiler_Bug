rust
extern {
    static FOO: u8;
}

const X: u8 = unsafe { FOO };
