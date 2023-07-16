
#[cfg(miri)]
{   // Miri does not support panics, ignore should_panic tests
    filtered.retain(|test| test.desc.should_panic == ShouldPanic::No);
}
