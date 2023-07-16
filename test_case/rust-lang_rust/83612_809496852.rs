javascript
{ let b = new Map(); searchIndex.std.i.map(function(v) {return [v[1].toLowerCase(), v[1], v[0]];}).filter(args => {let [lc, nc, v] = args; const k = lc+v; if (b.has(k)) { if (b.get(k) !== nc) { return true; } } b.set(k, nc); return false;});}
