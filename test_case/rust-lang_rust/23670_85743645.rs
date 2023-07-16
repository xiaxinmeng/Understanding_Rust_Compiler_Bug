 c++
#include <vector>
#include <cstdint>

extern "C" {
void cpp_version() {
    static std::vector<int32_t> v(100);
    for (int i = 0; i < 100; i++) {
        v.push_back(i);
    }
    v.resize(0);
}
}
