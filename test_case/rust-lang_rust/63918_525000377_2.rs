webassembly
;; local 1 = SP
;; local.2 = i64.load(sp + 16)
      get_local 1
      i32.const 16
      i32.add
      i64.load
      set_local 2
;; *out.0 = i64.load(sp + 8)
      get_local 0
      get_local 1
      i64.load offset=8
      i64.store
;; *(out.0+8) = local.2
      get_local 0
      get_local 2
      i64.store offset=8
