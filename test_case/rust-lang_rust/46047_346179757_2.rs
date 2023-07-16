
vcvarsall x64
cl /LD lib.c
cl main.c /link lib.lib
main.exe
echo %errorlevel% ; will print 0
move lib.dll lib1.dll
echo %errorlevel% ; will print non zero number
