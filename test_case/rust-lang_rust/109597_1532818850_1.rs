
a1 = *a;
b1 = *b;
a2 = *a;
assume(a1 == a2); // bytewise comparison of 2 locals, so no aliasing question
a2 = a1;
