rust
    if self.layout.for_variant(bcx.ccx, variant_index).abi == layout::Abi::Uninhabited {
        return;
    }
