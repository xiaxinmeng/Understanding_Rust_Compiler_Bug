 Rust
where
    P::HLI: 'a + 'b,
    P::PatternGenerator: 'a + 'b,
    P::PhraseGenerator: 'a + 'b,
    P::Instrument: 'a + 'b,
    <P::Instrument as Instrument>::Source: 'a + 'b,
    P::Effect: 'a + 'b,
    P::Sample: 'a + 'b,
    P::BusId: 'a + 'b,
