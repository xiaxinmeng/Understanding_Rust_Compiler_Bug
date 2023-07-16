c++
#include<vector>

std::size_t find_zero(std::vector<char>& v) {
    auto it = v.begin();
    for (; it != v.end(); ++it) {
        if (!*it) {
            break;
        }
    }
    return std::distance(v.begin(), it);
}
