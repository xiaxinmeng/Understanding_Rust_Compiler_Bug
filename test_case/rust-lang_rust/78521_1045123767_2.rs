
$ clang++ -Wall main.cpp
main.cpp:4:24: warning: all paths through this function will call itself [-Winfinite-recursion]
    int f(int x) const {
