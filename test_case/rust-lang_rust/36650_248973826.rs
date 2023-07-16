
# command-line-arguments
fatal error: unexpected signal during runtime execution
[signal 0xb code=0x1 addr=0xcf39d307ea2 pc=0xf47b]

runtime stack:
runtime.throw(0x2c4900, 0x2a)
    /usr/local/go/src/runtime/panic.go:547 +0x90
runtime.sigpanic()
    /usr/local/go/src/runtime/sigpanic_unix.go:12 +0x5a
runtime.unlock(0x3a60a0)
    /usr/local/go/src/runtime/lock_sema.go:107 +0x14b
runtime.(*mheap).alloc_m(0x3a60a0, 0x1, 0x40000000009, 0x513440)
    /usr/local/go/src/runtime/mheap.go:492 +0x314
runtime.(*mheap).alloc.func1()
    /usr/local/go/src/runtime/mheap.go:502 +0x41
runtime.systemstack(0xc82003be58)
    /usr/local/go/src/runtime/asm_amd64.s:307 +0xab
runtime.(*mheap).alloc(0x3a60a0, 0x1, 0x10000000009, 0xf11f)
    /usr/local/go/src/runtime/mheap.go:503 +0x63
runtime.(*mcentral).grow(0x3a7760, 0x0)
    /usr/local/go/src/runtime/mcentral.go:209 +0x93
runtime.(*mcentral).cacheSpan(0x3a7760, 0x49cdb0)
    /usr/local/go/src/runtime/mcentral.go:89 +0x47d
runtime.(*mcache).refill(0x401e10, 0xc800000009, 0x49cdb0)
    /usr/local/go/src/runtime/mcache.go:119 +0xcc
runtime.mallocgc.func2()
    /usr/local/go/src/runtime/malloc.go:642 +0x2b
runtime.systemstack(0xc82001c000)
    /usr/local/go/src/runtime/asm_amd64.s:291 +0x79
runtime.mstart()
    /usr/local/go/src/runtime/proc.go:1051

goroutine 1 [running]:
runtime.systemstack_switch()
    /usr/local/go/src/runtime/asm_amd64.s:245 fp=0xc8200386f8 sp=0xc8200386f0
runtime.mallocgc(0x80, 0x0, 0x3, 0x0)
    /usr/local/go/src/runtime/malloc.go:643 +0x869 fp=0xc8200387d0 sp=0xc8200386f8
runtime.rawstring(0x75, 0x0, 0x0, 0x0, 0x0, 0x0)
    /usr/local/go/src/runtime/string.go:284 +0x70 fp=0xc820038818 sp=0xc8200387d0
runtime.rawstringtmp(0x0, 0x75, 0x0, 0x0, 0x0, 0x0, 0x0)
    /usr/local/go/src/runtime/string.go:111 +0xb7 fp=0xc820038850 sp=0xc820038818
runtime.slicebytetostring(0x0, 0xc820959f80, 0x75, 0x75, 0x0, 0x0)
    /usr/local/go/src/runtime/string.go:93 +0x6f fp=0xc8200388e8 sp=0xc820038850
strings.Replace(0xc820afccb0, 0x61, 0x27cba0, 0x3, 0xc820038a00, 0x8, 0x4, 0x0, 0x0)
    /usr/local/go/src/strings/strings.go:675 +0x53d fp=0xc8200389b8 sp=0xc8200388e8
cmd/link/internal/ld.expandpkg(0xc820afccb0, 0x61, 0x27e550, 0x7, 0x0, 0x0)
    /usr/local/go/src/cmd/link/internal/ld/go.go:21 +0xb4 fp=0xc820038a28 sp=0xc8200389b8
cmd/link/internal/ld.rdsym(0xc820072240, 0xc820128ec0, 0x27e550, 0x7, 0x0)
    /usr/local/go/src/cmd/link/internal/ld/objfile.go:459 +0x1f3 fp=0xc820038b38 sp=0xc820038a28
cmd/link/internal/ld.readsym(0xc820072240, 0xc820128ec0, 0x27e550, 0x7, 0xc820122f00, 0x30)
    /usr/local/go/src/cmd/link/internal/ld/objfile.go:249 +0xd5b fp=0xc820038f80 sp=0xc820038b38
cmd/link/internal/ld.ldobjfile(0xc820072240, 0xc820128ec0, 0x27e550, 0x7, 0x2a19eb, 0xc820122f00, 0x30)
    /usr/local/go/src/cmd/link/internal/ld/objfile.go:147 +0xa62 fp=0xc820039190 sp=0xc820038f80
cmd/link/internal/ld.ldobj(0xc820128ec0, 0x27e550, 0x7, 0x2a1a45, 0xc820122f00, 0x30, 0xc820012510, 0x28, 0x1, 0x0)
    /usr/local/go/src/cmd/link/internal/ld/lib.go:1351 +0x1569 fp=0xc820039400 sp=0xc820039190
cmd/link/internal/ld.objfile(0xc82001a460)
    /usr/local/go/src/cmd/link/internal/ld/lib.go:847 +0x10d9 fp=0xc820039710 sp=0xc820039400
cmd/link/internal/ld.loadlib()
    /usr/local/go/src/cmd/link/internal/ld/lib.go:513 +0x5ce fp=0xc8200399f0 sp=0xc820039710
cmd/link/internal/ld.Ldmain()
    /usr/local/go/src/cmd/link/internal/ld/pobj.go:214 +0x1cd3 fp=0xc820039e70 sp=0xc8200399f0
cmd/link/internal/amd64.Main()
    /usr/local/go/src/cmd/link/internal/amd64/obj.go:44 +0x19 fp=0xc820039e78 sp=0xc820039e70
main.main()
    /usr/local/go/src/cmd/link/main.go:27 +0x36f fp=0xc820039f50 sp=0xc820039e78
runtime.main()
    /usr/local/go/src/runtime/proc.go:188 +0x2b0 fp=0xc820039fa0 sp=0xc820039f50
runtime.goexit()
    /usr/local/go/src/runtime/asm_amd64.s:1998 +0x1 fp=0xc820039fa8 sp=0xc820039fa0
make[2]: *** [bin/llvm-go] Error 2
make[1]: *** [tools/llvm-go/CMakeFiles/llvm-go.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
