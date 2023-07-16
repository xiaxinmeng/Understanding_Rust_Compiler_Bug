cpp
// C++
// Compile: `cl /EHsc /nologo argv.cpp`
#include <iostream>
int wmain(int argc, const wchar_t* wargv[])
{
    for (int i = 0; i < argc; i++) {
        std::wcout << '`' << wargv[i] << '`' << std::endl;
    }
}
