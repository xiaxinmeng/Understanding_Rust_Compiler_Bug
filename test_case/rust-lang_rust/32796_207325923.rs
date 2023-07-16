
if(
#ifdef A
a
#endif
#if defined(A) && defined(B)
&&
#endif
#if defined(B)
b
#endif
#if defined(A) || defined(B)
&&
#endif
c)
{}
