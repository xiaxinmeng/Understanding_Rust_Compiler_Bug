rust
dbg! (a) : typeof(a)
     (a) : typeof(a)

dbg! (a,) : (typeof(a),) // note the one-tuple type
     (a,) : (typeof(a),) // note the one-tuple type

dbg! (a, b) : (typeof(a), typeof(b))
     (a, b) : (typeof(a), typeof(b))

dbg! (a, b, c) : (typeof(a), typeof(b), typeof(c))
     (a, b, c) : (typeof(a), typeof(b), typeof(c))
