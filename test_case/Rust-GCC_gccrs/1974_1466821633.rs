
__attribute__((cdecl))
void test::main ()
{
  i32 D.88;

  _1 = 0;
  D.88 = _1;
  RUSTTMP.2 = test::foo (&D.88);
}


__attribute__((cdecl))
bool test::foo (const i32 & const RSTPRM.0)
{
  bool D.89;

  _1 = *RSTPRM.0;
  D.89 = _1 == 0;
  return D.89;
}
