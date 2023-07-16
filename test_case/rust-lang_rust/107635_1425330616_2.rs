
  (func $_start (;9;) (type 2)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load offset=1353400
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store offset=1353400
        call $__wasm_call_ctors
        call $__main_void
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
