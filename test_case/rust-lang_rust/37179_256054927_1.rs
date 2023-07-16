
  FuncId (0x10B4) {
    TypeLeafKind: LF_FUNC_ID (0x1601)
    ParentScope: primes (0x10B2)
    FunctionType: __int64 () (0x10B3)
    Name: compute_primes
  }
...
CodeViewDebugInfo [
  Subsection [
    SubSectionType: InlineeLines (0xF6)
    InlineeSourceLine {
      Inlinee: compute_primes (0x10B4)
      FileID: C:\proj\r\test-msvc\primes.rs (0x30)
      SourceLineNum: 3
    }
...
CodeViewDebugInfo [
  Subsection [
    SubSectionType: Symbols (0xF1)
    ProcStart {
      Kind: S_LPROC32_ID (0x1146)
      FunctionType: do_call (0x11EE)
      CodeOffset: _ZN3std9panicking3try7do_call17h7aa81a0e201766daE+0x0
      DisplayName: std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,i64>
      LinkageName: _ZN3std9panicking3try7do_call17h7aa81a0e201766daE
    }
...
    InlineSite {
>     Inlinee: compute_primes (0x10B4)
      BinaryAnnotations [
        ChangeLineOffset: 6
        ChangeCodeOffset: 0x20
        ChangeCodeLength: 0x6
        ChangeLineOffset: 1
        ChangeCodeOffset: 0x10
        ChangeLineOffset: 1
        ChangeCodeOffset: 0x12
...
    }
    Local {
      Type: __int64 (0x13)
      Flags [ (0x0) ]
      VarName: a
    }
...
    Local {
      Type: __int64 (0x13)
      Flags [ (0x100) IsOptimizedOut (0x100)
      ]
      VarName: count
    }
...
