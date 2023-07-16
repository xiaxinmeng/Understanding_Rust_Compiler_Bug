cpp
#include<vector>

void map_on_elems(const std::vector<int> &vec, void(&func)(int)) {
    for(std::size_t i = 0; i < vec.size(); ++i) {
     func(vec.at(i));
    }
}
