cpp
#if defined(_WIN32) && !defined(_WIN64)
#    define VS_CC __stdcall
#else
#    define VS_CC
#endif
