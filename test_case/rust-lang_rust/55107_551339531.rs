
function testround()
  for u = typemin(UInt32):typemax(UInt32)
    x = reinterpret(Float32, u)
    k = 2/eps(Float32)
    y = copysign((abs(x)-k)+k, x)
    if !isequal(round(x), y)
        @show x
        return false
    end
  end
  return true
end
