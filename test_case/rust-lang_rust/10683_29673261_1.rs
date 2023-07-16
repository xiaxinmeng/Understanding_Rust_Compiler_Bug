
   49d9c:       e8 0f 68 fe ff          callq  305b0 <_ZN5ascii26StrAsciiExt$__extensions__14to_ascii_lower19h2742b85a3a278c3GaF4v0.1E>
   49da1:       48 89 45 b0             mov    %rax,-0x50(%rbp)
   49da5:       48 89 45 a8             mov    %rax,-0x58(%rbp)
   49da9:       48 8d 7d b8             lea    -0x48(%rbp),%rdi
   49dad:       48 8d 75 a8             lea    -0x58(%rbp),%rsi
   49db1:       e8 ea 76 fe ff          callq  314a0 <_ZN3str18Str$__extensions__8as_slice20h13b4445a0db8813U4ak4v0.1E>
   49db6:       eb 00                   jmp    49db8 <_ZN10properties19PropertyDeclaration5parse19h586366822340e1rhab4v0.1E+0x88>
   49db8:       48 bf 00 00 00 00 00    movabs $0x0,%rdi
   49dbf:       00 00 00 
   49dc2:       48 8d 75 a8             lea    -0x58(%rbp),%rsi
   49dc6:       e8 15 1e fd ff          callq  1bbe0 <_ZN8_$UP$str9glue_drop19h62ec8c421db32e66a5E>
   49dcb:       48 8d 75 b8             lea    -0x48(%rbp),%rsi
   49dcf:       48 8d 55 88             lea    -0x78(%rbp),%rdx
   49dd3:       48 8d 05 da a1 09 00    lea    0x9a1da(%rip),%rax        # e3fb4 <str8205>
   49dda:       48 89 45 88             mov    %rax,-0x78(%rbp)
   49dde:       48 c7 45 90 0a 00 00    movq   $0xa,-0x70(%rbp)
   49de5:       00 
   49de6:       e8 35 77 fe ff          callq  31520 <_ZN3str8eq_slice19h6fa631b3d59b91e0aU4v0.1E>
   49deb:       3c 00                   cmp    $0x0,%al
   49ded:       0f 85 0e 15 00 00       jne    4b301 <_ZN10properties19PropertyDeclaration5parse19h586366822340e1rhab4v0.1E+0x15d1>
   49df3:       e9 0e 15 00 00          jmpq   4b306 <_ZN10properties19PropertyDeclaration5parse19h586366822340e1rhab4v0.1E+0x15d6>
