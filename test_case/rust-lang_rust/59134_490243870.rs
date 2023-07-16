rust
struct BF { bits: u32 }

impl BF {
    pub const F: BF = BF { bits: 0 };
    pub const fn f() {
        trait _BF {
            const FLAG: u32 = 0;
        }

        impl _BF for BF {
            const FLAG: u32 = F.bits;
        }
    }
}
