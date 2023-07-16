
C:\Users\micha\testexe>.\target\debug\testexe.exe "a ""b"" c"
env::args() = [".\\target\\debug\\testexe.exe", "a \"b", "c"]
GetCommandLineW = .\target\debug\testexe.exe  "a ""b"" c"

C:\Users\micha\testexe>.\target\debug\testexe.exe "a """b""" c"
env::args() = [".\\target\\debug\\testexe.exe", "a \"b\" c"]
GetCommandLineW = .\target\debug\testexe.exe  "a """b""" c"

C:\Users\micha\testexe>.\target\debug\testexe.exe "a \"b\" c"
env::args() = [".\\target\\debug\\testexe.exe", "a \"b\" c"]
GetCommandLineW = .\target\debug\testexe.exe  "a \"b\" c"

C:\Users\micha\testexe>.\target\debug\testexe.exe "a "\^"b\^"" c"
env::args() = [".\\target\\debug\\testexe.exe", "a \"b\" c"]
GetCommandLineW = .\target\debug\testexe.exe  "a "\"b\"" c"

C:\Users\micha\testexe>
