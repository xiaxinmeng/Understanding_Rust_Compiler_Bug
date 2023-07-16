
  FuncId (0x1919) {
    TypeLeafKind: LF_FUNC_ID (0x1601)
    ParentScope: 0x0
    FunctionType: unsigned __int64 () (0x164C)
    Name: compute_primes
  }
...
CodeViewDebugInfo [
  Subsection [
    SubSectionType: Symbols (0xF1)
    ProcStart {
      Kind: S_LPROC32_ID (0x1146)
      FunctionType: compute_primes (0x1919)
      CodeOffset: ?compute_primes@@YA_KXZ+0x0
      Flags [ (0x80)
        HasOptimizedDebugInfo (0x80)
      ]
      DisplayName: compute_primes
      LinkageName: ?compute_primes@@YA_KXZ
    }
...
]
CodeViewDebugInfo [
  Subsection [
    SubSectionType: Symbols (0xF1)
    ProcStart {
      Kind: S_GPROC32_ID (0x1147)
      FunctionType: thread_some_primes (0x191A)
      CodeOffset: ?thread_some_primes@@YAXXZ+0x0
      Flags [ (0x80)
        HasOptimizedDebugInfo (0x80)
      ]
      DisplayName: thread_some_primes
      LinkageName: ?thread_some_primes@@YAXXZ
    }
    InlineSite {
      Inlinee: compute_primes (0x1919)
      BinaryAnnotations [
        ChangeCodeOffsetAndLineOffset: {CodeOffset: 0x0, LineOffset: 0}
        ChangeLineOffset: 1
...
