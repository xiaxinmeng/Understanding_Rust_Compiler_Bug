
PS D:\git\cad97\playground> cargo +beta test -- --include-ignored > $null
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src\lib.rs (D:\.rust\target\debug\deps\playground-917c13ed19ceedcc.exe)
     Running unittests src\main.rs (D:\.rust\target\debug\deps\playground-3a8e8a709e4658bf.exe)
   Doc-tests playground
PS D:\git\cad97\playground> cargo +nightly test -- --include-ignored > $null
   Compiling playground v0.0.0 (D:\git\cad97\playground)
    Finished test [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src\lib.rs (D:\.rust\target\debug\deps\playground-f1421a85aae244d2.exe)
     Running unittests src\main.rs (D:\.rust\target\debug\deps\playground-cc3e84b5b419fbbd.exe)
