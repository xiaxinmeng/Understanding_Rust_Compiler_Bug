
    bb6: {
        // Some(_) if ... branch
        _5 = ((_2 as Some).0: &str);
        StorageDead(_2);                         // no longer used...
        _9 = _5;
        ((_0 as Some).0: &str) = move _9;
        discriminant(_0) = 1;
        goto -> bb8;
    }

    bb7: {
        StorageDead(_2);
        // this is the last Some(_) branch...
        // no borrows to `self` or the return value of `Test::test` are live at this point, free to call whatever user wishes to.
        goto -> bb8;
    }

    bb8: {
        return;
    }
