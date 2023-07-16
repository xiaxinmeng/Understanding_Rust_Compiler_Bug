rust
.filter(divisible_by_three.or(divisible_by_five))
.filter(or(divisible_by_three, divisible_by_five))
.filter(divisible_by_three || divisible_by_five)
