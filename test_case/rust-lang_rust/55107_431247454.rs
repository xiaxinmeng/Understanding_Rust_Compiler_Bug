
k = 1/eps(Float32)
a = abs(x)
a < k ? copysign((a+k)-k,x) : x
