rust
for<'x> <() as Trait<'x>>::A == ()
for<'x> <&&() as Trait<'x>>::A == ()
    for<'x> <&() as Trait<'x>>::A == ()
        for<'x> <() as Trait<'x>>::A == ()
for<'x> <&() as Trait<'x>>::A == ()
    for<'x> <() as Trait<'x>>::A == ()
