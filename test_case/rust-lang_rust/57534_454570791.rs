cpp
#include <iostream>

struct A {
    int x;
    ~A();
};

thread_local A thread_a{3557};

A::~A() {
    static thread_local std::string thread_string{"hello there long enough string"};

    // next line triggers a call to _tlv_atexit during tlv_finalize, and makes ASAN unhappy
    std::cout << thread_string << std::endl;
}

int main() {
    std::cout << thread_a.x << std::endl;
    return 0;
}
