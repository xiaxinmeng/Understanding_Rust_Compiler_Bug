
        // If the static allocation is mutable or if it has relocations (it may be legal to mutate
        // the memory behind that in the future), then we can't const prop it.
