c
i32 f (const i32 a)
{
  i32 D.204;

  _1 = a + 1;
  if (_1 == -2147483648) goto <D.201>; else goto <D.202>;
  <D.201>:
  {
    RUSTTMP.1 = 1;
  }
  goto <D.203>;
  <D.202>:
  {
    RUSTTMP.1 = 0;
  }
  <D.203>:
  D.204 = RUSTTMP.1;
  return D.204;
}
