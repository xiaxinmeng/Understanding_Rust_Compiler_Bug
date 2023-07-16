rust
    bb0: {
        StorageLive(_2);
        _2 = core::num::<impl i64>::max_value() -> bb1;
    }

    bb1: {
        _3 = Eq(const 2i64, const 0i64);
        assert(!_3, "attempt to calculate the remainder with a divisor of zero") -> bb2;
    }

    bb2: {
        _4 = Eq(const 2i64, const -1i64);
        _5 = Eq(_2, const -9223372036854775808i64);
        _6 = BitAnd(_4, _5);
        assert(!_6, "attempt to calculate the remainder with overflow") -> bb3;
    }

    bb3: {
        _1 = Rem(_2, const 2i64);
        StorageDead(_2);
    }
