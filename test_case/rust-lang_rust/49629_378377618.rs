rust
a::b::c!(tt*)
a::b::c![tt*]
a::b::c!{tt*}

=>

#[a::b::c(tt*)]
#[a::b::c[tt*]]
#[a::b::c{tt*}]
