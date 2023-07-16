 diff
- #if defined(_MSC_VER)
+ #if defined(_MSC_VER) || defined(__MINGW64_VERSION_MAJOR)
   swprintf(path2, len + 3, fmt, pathw);
 #else
   swprintf(path2, fmt, pathw);
 #endif
