
__attribute__((cdecl))
void ref_pattern_fn_param2::main ()
{
  i32 D.88;

  _1 = 0;
  D.88 = _1;
  RUSTTMP.1 = ref_pattern_fn_param2::foo (&D.88);
}


__attribute__((cdecl))
bool ref_pattern_fn_param2::foo (const i32 b)
{
  bool D.89;

  D.89 = b == 0;
  return D.89;
}
