
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" <snip>
  = note: 0  0x10580a2c0  __assert_rtn + 129
          1  0x1058d1eb3  ld::passes::tlvp::doPass(Options const&, ld::Internal&) + 1987
          2  0x10580b16b  main + 913
          A linker snapshot was created at:
              /tmp/1-2018-05-28-205411.ld-snapshot
          ld: Assertion failed: (0 && "wrong content type for target in tlv defs"), function doPass, file /Library/Caches/com.apple.xbs/Sources/ld64/ld64-351.8/src/ld/passes/tlvp.cpp, line 293.
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
