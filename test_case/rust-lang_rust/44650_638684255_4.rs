
notriddle@DESKTOP-20BNT14:/mnt/c/Users/micha/testexe$ ./target/debug/testexe.exe "a \"b\" c"
env::args() = ["C:\\Users\\micha\\testexe\\target\\debug\\testexe.exe", "a \"b\" c"]
GetCommandLineW = C:\Users\micha\testexe\target\debug\testexe.exe "a \"b\" c"
notriddle@DESKTOP-20BNT14:/mnt/c/Users/micha/testexe$ ./target/debug/testexe.exe "a ""b"" c"
env::args() = ["C:\\Users\\micha\\testexe\\target\\debug\\testexe.exe", "a b c"]
GetCommandLineW = C:\Users\micha\testexe\target\debug\testexe.exe "a b c"
notriddle@DESKTOP-20BNT14:/mnt/c/Users/micha/testexe$ ./target/debug/testexe.exe "a """b""" c"
env::args() = ["C:\\Users\\micha\\testexe\\target\\debug\\testexe.exe", "a b c"]
GetCommandLineW = C:\Users\micha\testexe\target\debug\testexe.exe "a b c"
notriddle@DESKTOP-20BNT14:/mnt/c/Users/micha/testexe$
