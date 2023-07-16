 rust
#include<random>
#include<iostream>

int main() {
    std::default_random_engine r(0);
    std::default_random_engine r2(r);

    std::cout << r() << " " << r2() << std::endl; // 16807 16807
}
