
fn@ func1(x: int) -> int {
  if (x == 0) { -1 } else { func2(x-1) }
}
and fn@ func2 (x: int) -> int {
  if (x % 3 == 0) { x * 5 } else { func1(x-1) }
}
