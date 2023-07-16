rust
if false {
    // This drop now does run "unconditionally" in the sense of there being no more
    // initializedness check, but of course it does not *actually* run because of `if false`
    Drop(_5);
}

StorageLive(_5);
_5 = something;
other_thing = _5;
StorageDead(_5);
