
let foo = @mut none, bar = @mut none;
foo = some(fn@ { ... use bar.get() ... })
bar = some(fn@ { ... use foo.get() ... })
