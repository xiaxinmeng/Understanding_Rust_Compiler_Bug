
test [compile-fail] compile-fail/circular_modules_hello.rs ... ignored
prefix=c:\code\rust\src\test\compile-fail\circular_modules_main.rs:12: ee.kind=error: ee.msg=circular modules line=C:\code\rust\src\test\compile-fail\circular_modules_main.rs:12:4:
 12:26 error: circular modules: C:\code\rust\src\test\compile-fail\circular_modules_hello.rs -> C:\code\rust\src\test\compile-fail\circular_modules_main.rs -> C:\code\rust\src\test
prefix=c:\code\rust\src\test\compile-fail\circular_modules_main.rs:12: ee.kind=error: ee.msg=circular modules line=C:\code\rust\src\test\compile-fail\circular_modules_main.rs:12 mo
prefix=c:\code\rust\src\test\compile-fail\circular_modules_main.rs:12: ee.kind=error: ee.msg=circular modules line=
  ^~~~~~~~~~~~~~~~~~~~~~
error: expected error: on line 12 not found: circular modules
command: PATH="i686-pc-mingw32/stage1/bin;;.;C:\Apps\MinGW\msys\1.0\local\bin;C:\Apps\MinGW\bin;C:\Apps\MinGW\msys\1.0\bin;c:\Program Files\Common Files\Microsoft Shared\Windows Li
ve;c:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;c:\Windows\system32;c:\Windows;c:\Windows\System32\Wbem;c:\Windows\System32\WindowsPowerShell\v1.0\;c:\Program Files\Torto
iseHg\;c:\Program Files\Microsoft SQL Server\110\Tools\Binn\;c:\Program Files\Perforce;c:\Program Files (x86)\CMake 2.8\bin;c:\Program Files (x86)\GtkSharp\2.12\bin;C:\Apps\MinGW\b
in;c:\Program Files (x86)\Git\cmd;c:\Program Files\Common Files\Microsoft Shared\Windows Live;c:\Program Files\Perforce\Server;i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin"
 i686-pc-mingw32\stage1\bin\rustc.exe c:\code\rust\src\test\compile-fail\circular_modules_main.rs -o i686-pc-mingw32\test\compile-fail\circular_modules_main.stage1-i686-pc-mingw32.
exe -L i686-pc-mingw32\test\compile-fail -L i686-pc-mingw32\test\compile-fail\circular_modules_main.libaux -O --target=i686-pc-mingw32
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
C:\code\rust\src\test\compile-fail\circular_modules_main.rs:12:4: 12:26 error: circular modules: C:\code\rust\src\test\compile-fail\circular_modules_hello.rs -> C:\code\rust\src\te
st\compile-fail\circular_modules_main.rs -> C:\code\rust\src\test\compile-fail\circular_modules_hello.rs
C:\code\rust\src\test\compile-fail\circular_modules_main.rs:12 mod circular_modules_hello; //~ERROR: circular modules
                                                                   ^~~~~~~~~~~~~~~~~~~~~~
