rust
// _5 is not initialized here, so this drop does not actually execute at runtime,
// but still it is an ICE because of a use outside of storage liveness
Drop(_5);

StorageLive(_5);
_5 = something;
other_thing = _5;
StorageDead(_5);
