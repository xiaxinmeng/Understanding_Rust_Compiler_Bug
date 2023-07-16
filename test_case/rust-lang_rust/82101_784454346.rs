c
typedef char Char[3] __attribute__((aligned(2)));

struct X {
	Char c[3];
} x;

int main(void) {
	_Static_assert(sizeof(x.c) == 10, "");
	_Static_assert(__builtin_offsetof(struct X, c[1]) == 3, "");
	_Static_assert(_Alignof(Char) == 2, "");
}
