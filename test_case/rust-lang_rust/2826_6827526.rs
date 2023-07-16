
let x: @int = @0;
let y: ~int = ~0;
// I would expect this
let z: *int = *0;
// But is this what we're going to get?
let z: *int = &0;
