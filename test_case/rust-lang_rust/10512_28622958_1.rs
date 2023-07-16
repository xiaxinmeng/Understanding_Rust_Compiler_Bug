
windres -i resource.rc -o resource.o
gcc -c main.c -o main.o
gcc -o app.exe main.o resource.o -s -Wl,--subsystem,windows
