rust
unsafe {
    // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
    let rcc = &(*RCC::ptr());

    // Enable clock.
    $GPIOX::enable(rcc);
    $GPIOX::reset(rcc);
}
