
// temp.d(2,33): Error: cannot implicitly convert expression (x) of type const(uint) to ubyte
ubyte foo01(in uint x) { return x; }

ubyte foo02(in uint x) { return x % 300; } // Cannot implicitly convert
ubyte foo03(in uint x) { return x % 200; } // OK
ubyte foo04(in int x) { return x % 200; } // Cannot implicitly convert
ubyte foo05(in int x) { return x % 128 + 128; } // OK
byte foo06(in int x) { return x % 127; } // OK
byte foo07(in ubyte x) { return x - 128; } // OK
int foo08(in ubyte x) { return x - 128; } // OK
ubyte foo09(in uint x) { return x & 0b_1111; } // OK
ubyte foo10(in uint x) { return x / 10_000_000; } // Cannot implicitly convert
ubyte foo11(in uint x) { return x / 100_000_000; } // OK
ubyte foo12(in ubyte x, in ubyte y) { return x + y; } // Cannot implicitly convert
uint foo13(in ubyte x, in ubyte y) { return x + y; } // OK

void main() {}
