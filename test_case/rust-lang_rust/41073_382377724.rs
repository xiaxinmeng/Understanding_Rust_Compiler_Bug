
   union int_or_short { int x; short y; } u = { .x = 3 };
   printf("%d\n", u.y);
