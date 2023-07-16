
__attribute__((cdecl))
void ref_pattern_fn_param2::main ()
{
  RUSTTMP.1 = ref_pattern_fn_param2::foo (0);
}


__attribute__((cdecl))
bool ref_pattern_fn_param2::foo (const i32 b)
{
  bool D.88;

  D.88 = b == 0;
  return D.88;
}
