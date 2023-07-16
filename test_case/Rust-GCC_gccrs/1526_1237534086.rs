c
__attribute__((cdecl))
u64 const_eval_oeis_a006884::test ()
{
  u64 n = 113383;

    u64 RUSTTMP.0;
    u64 n = 113383;
  while (1)
    {
      {
        <D.82>:;
        if (n == 1) break;
        {
                    u64 RUSTTMP.1;
          if ((n & 1) == 0)
            {
              {
                RUSTTMP.1 = n / 2;
              }
            }
          else
            {
              {
                RUSTTMP.1 = n * 3 + 1;
              }
            }
          n = RUSTTMP.1;
        }
      }
    }
  return n;
}
