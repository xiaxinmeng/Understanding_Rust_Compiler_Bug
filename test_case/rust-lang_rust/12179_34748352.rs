 rust
macro_rules! breakfoo( () => { break 'foo; } )
'foo: for x in bar() { breakfoo!() }
