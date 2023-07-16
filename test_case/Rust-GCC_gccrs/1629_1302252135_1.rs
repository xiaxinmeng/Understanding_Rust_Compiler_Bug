c
__attribute__((cdecl))
i32 atomic_cmp_xchg::main ()
{
  complex int D.219;
  complex int D.222;
  complex int D.225;
  i32 D.228;
  const struct (i32, bool) res0;
  const struct (i32, bool) res1;

  try
    {
      res0 = atomic_cmp_xchg::perform_call (15);
      res1 = atomic_cmp_xchg::perform_call (189);
      _1 = res0.__1;
      if (_1 != 0) goto <D.217>; else goto <D.215>;
      <D.217>:
      _2 = res1.__1;
      _3 = ~_2;
      if (_3 != 0) goto <D.218>; else goto <D.215>;
      <D.218>:
      {
        _4 = res0.__0;
        D.219 = .SUB_OVERFLOW (_4, 15);
        _5 = REALPART_EXPR <D.219>;
        RUSTTMP.6 = _5;
        _6 = IMAGPART_EXPR <D.219>;
        _7 = (bool) _6;
        if (_7 != 0) goto <D.220>; else goto <D.221>;
        <D.220>:
        __builtin_abort ();
        <D.221>:
        _8 = res1.__0;
        D.222 = .SUB_OVERFLOW (_8, 15);
        _9 = REALPART_EXPR <D.222>;
        RUSTTMP.7 = _9;
        _10 = IMAGPART_EXPR <D.222>;
        _11 = (bool) _10;
        if (_11 != 0) goto <D.223>; else goto <D.224>;
        <D.223>:
        __builtin_abort ();
        <D.224>:
        RUSTTMP.9_12 = RUSTTMP.6;
        RUSTTMP.10_13 = RUSTTMP.7;
        D.225 = .ADD_OVERFLOW (RUSTTMP.9_12, RUSTTMP.10_13);
        _14 = REALPART_EXPR <D.225>;
        RUSTTMP.8 = _14;
        _15 = IMAGPART_EXPR <D.225>;
        _16 = (bool) _15;
        if (_16 != 0) goto <D.226>; else goto <D.227>;
        <D.226>:
        __builtin_abort ();
        <D.227>:
        RUSTTMP.5 = RUSTTMP.8;
      }
      goto <D.216>;
      <D.215>:
      {
        RUSTTMP.5 = 1;
      }
      <D.216>:
      D.228 = RUSTTMP.5;
      return D.228;
    }
  finally
    {
      res0 = {CLOBBER(eol)};
      res1 = {CLOBBER(eol)};
    }
}


__attribute__((cdecl))
struct (i32, bool) atomic_cmp_xchg::perform_call (const i32 old)
{
  struct (i32, bool) D.231;
  i32 dst;
  const i32 new;
  const struct (T=i32, bool) res;

  try
    {
      dst = 15;
      new = 14;
      {
        RUSTTMP.1 = atomic_cxchg_seqcst_seqcst<i32> (&dst, old, new);
      }
      res = RUSTTMP.1;
      _1 = res.__0;
      _2 = res.__1;
      D.231.__0 = _1;
      D.231.__1 = _2;
      return D.231;
    }
  finally
    {
      dst = {CLOBBER(eol)};
      res = {CLOBBER(eol)};
    }
}


struct (T=i32, bool) atomic_cxchg_seqcst_seqcst<i32> (i32 * const dst, const i32 old, const i32 src)
{
  struct (T=i32, bool) D.234;

  RUSTTMP.2 = old;
  RUSTTMP.3 = __atomic_compare_exchange_4 (dst, &RUSTTMP.2, src, 0, 5, 5);
  RUSTTMP.11_1 = RUSTTMP.2;
  D.234.__0 = RUSTTMP.11_1;
  D.234.__1 = RUSTTMP.3;
  return D.234;
}
