c++
#include <optional>
#include <cstdint>

std::optional<uint32_t> foo1(uint32_t x, size_t i){
    static const uint32_t PS[6] = {2, 3, 5, 7, 11, 13};
    if (i < 6) {
        uint32_t psi = PS[i];
        if (psi == 0) throw "divisor zero";
        return std::optional(x % psi);
    } else {
        return std::nullopt;
    }
}
