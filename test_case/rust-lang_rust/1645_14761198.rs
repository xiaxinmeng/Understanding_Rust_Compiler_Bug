 C
union {
   struct { some_int_t discr; /* args of variant 1 */ } v1;
   /* ... */
   struct { some_int_t discr; /* args of variant N */ } vN;
};
