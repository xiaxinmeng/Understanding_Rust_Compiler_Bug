C
va_list ap1, ap2;
va_copy(ap1, ap);
if (condition)
  va_copy(ap2, ap);
// ...other code...
va_end(ap1);
// ...more code...
if (same_condition)
  va_end(ap2);
