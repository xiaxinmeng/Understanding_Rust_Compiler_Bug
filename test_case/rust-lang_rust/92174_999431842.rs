cpp
#include <cstddef>
#include <vector>

void do_checks(const std::vector<int> v){
    size_t idx = 0;
    for(auto& item : v){
        v.at(idx);
        ++idx;
    }
}
