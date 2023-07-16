c
struct (T=i32, bool) atomic_cxchg_seqcst_seqcst<i32> (i32 * const dst, const i32 old, const i32 src)
{
    i32 RUSTTMP.2 = (i32) old;
    bool RUSTTMP.3 = __atomic_compare_exchange_4 (dst, &RUSTTMP.2, src, 0, 5, 5);
  return {.__0=RUSTTMP.2, .__1=RUSTTMP.3};
}


__attribute__((cdecl))
struct (i32, bool) atomic_cmp_xchg::perform_call (const i32 old)
{
  i32 dst = 15;
  const i32 new = 14;
  const struct (T=i32, bool) res = RUSTTMP.1;

    struct (i32, bool) RUSTTMP.0;
    i32 dst = 15;
    const i32 new = 14;
    struct (T=i32, bool) RUSTTMP.1;
  {
    RUSTTMP.1 = atomic_cxchg_seqcst_seqcst<i32> (&dst, old, new);
  }
    const struct (T=i32, bool) res = RUSTTMP.1;
  return {.__0=res.__0, .__1=res.__1};
}


__attribute__((cdecl))
i32 atomic_cmp_xchg::main ()
{
  const struct (i32, bool) res0 = atomic_cmp_xchg::perform_call (15);
  const struct (i32, bool) res1 = atomic_cmp_xchg::perform_call (189);

    i32 RUSTTMP.4;
    const struct (i32, bool) res0 = atomic_cmp_xchg::perform_call (15);
    const struct (i32, bool) res1 = atomic_cmp_xchg::perform_call (189);
    i32 RUSTTMP.5;
  if (res0.__1 && !res1.__1)
    {
      {
        if ((RUSTTMP.6 = REALPART_EXPR <SAVE_EXPR <.SUB_OVERFLOW (res0.__0, 15)>>;, (bool) IMAGPART_EXPR <SAVE_EXPR <.SUB_OVERFLOW (res0.__0, 15)>>;) == 1)
          {
            __builtin_abort ();
          }
        if ((RUSTTMP.7 = REALPART_EXPR <SAVE_EXPR <.SUB_OVERFLOW (res1.__0, 15)>>;, (bool) IMAGPART_EXPR <SAVE_EXPR <.SUB_OVERFLOW (res1.__0, 15)>>;) == 1)
          {
            __builtin_abort ();
          }
        if ((RUSTTMP.8 = REALPART_EXPR <SAVE_EXPR <.ADD_OVERFLOW (RUSTTMP.6, RUSTTMP.7)>>;, (bool) IMAGPART_EXPR <SAVE_EXPR <.ADD_OVERFLOW (RUSTTMP.6, RUSTTMP.7)>>;) == 1)
          {
            __builtin_abort ();
          }
        RUSTTMP.5 = RUSTTMP.8;
      }
    }
  else
    {
      {
        RUSTTMP.5 = 1;
      }
    }
  return RUSTTMP.5;
}



