 c
// a.c
void a() {}

// b.c
extern void a();
void b() { a(); }

// c.c
extern void b();
int main() { b(); }
