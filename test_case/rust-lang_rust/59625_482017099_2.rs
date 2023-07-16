C
va_list aq, ap1, ap2;
// ...initialize ap1 and ap2...
if (condition) {
  va_copy(aq, ap1);
} else {
  va_copy(aq, ap2);
}
// ...
va_end(aq);
