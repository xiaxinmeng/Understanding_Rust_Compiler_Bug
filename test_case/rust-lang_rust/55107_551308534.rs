
julia> x = 1/eps(Float32)+1
8.388609f6

julia> isinteger(x)
true

julia> k = 1/eps(Float32)
8.388608f6

julia> copysign((abs(x)+k)-k, x)
8.388608f6
