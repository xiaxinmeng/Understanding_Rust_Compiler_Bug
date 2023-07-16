
// gmp: mpz_add(x, y, z)
x = y + z // to initialize x
x.set_add(y, z) // to re-use memory allocated to x
// gmpxx uses an operator= overload using template hackery to do this for simple cases
