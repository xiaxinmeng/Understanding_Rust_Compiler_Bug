
    bbN: {
        StorageLive(_X);
        _Y = call of some sort -> [return: bbN+1, unwind: bbPoisoned];
    }

    bbN+1: {
        StorageDead(_X);
        goto -> bbN+2;
    }

    bbPoisoned (cleanup): {
        discriminant(Generator) = 2;
        resume;
    }
