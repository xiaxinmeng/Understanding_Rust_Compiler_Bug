
    bbN: {
        StorageLive(_X);
        _Y = call of some sort -> [return: bbN+1];
    }

    bbN+1: {
        StorageDead(_X);
        goto -> bbN+2;
    }
