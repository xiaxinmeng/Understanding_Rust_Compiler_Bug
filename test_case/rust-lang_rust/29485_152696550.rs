 c++
#include <iostream>
using namespace std;

void xx() { 
    throw 1;
}

void (*volatile g)() = xx;

void f(int* __restrict x) {
    *x = 1;
    g();
    *x = 0;
}

int main() {
    int x = 0;
    try { f(&x); } catch(...) {}
    std::cout << x;
    return 0;
}
