rust
    fn check10() {
        macro m() {}
        {
            macro m() {}
            gen_invoc!(); // OK
        }
    }
