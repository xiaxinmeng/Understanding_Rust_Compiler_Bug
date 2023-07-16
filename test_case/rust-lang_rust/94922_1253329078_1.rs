
sum(x, y) = [x ≤ y] ⋅ {
 ⌊(x + y) ÷ 2⌋ ⋅ (y - x + 1), if 2 | (x + y),
 (x + y) ⋅ ⌊(y - x + 1) ÷ 2⌋, else
}
