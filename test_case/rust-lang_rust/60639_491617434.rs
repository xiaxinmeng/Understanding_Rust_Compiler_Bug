c
char *ptr = malloc(42); // assert no null
char *ub = ptr + 43;
char *also_ub = ptr - 1;
