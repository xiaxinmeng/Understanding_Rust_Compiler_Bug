
void main ()
{
  struct (i32,bool) a;
  struct (i32,f32) b;

  try
    {
      a.0 = 123;
      a.1 = 1;
      b.0 = 456;
      b.1 = 5.0e+0;
    }
  finally
    {
      a = {CLOBBER};
      b = {CLOBBER};
    }
}
