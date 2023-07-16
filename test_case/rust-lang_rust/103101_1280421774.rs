c
char* ptr = malloc(...); // Equivalent: PanicGuard::new()
// ...
free(ptr); // Equivalent: forget(guard). If you don't put this in, you're sad in C but much sadder with the guard because it's a UAF.
