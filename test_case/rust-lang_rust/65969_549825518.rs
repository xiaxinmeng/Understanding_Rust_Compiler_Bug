
#include <vector>

int foo(std::vector<int> const& a, std::vector<int> const& b) {
    int index = 0;
    int x = 0;
    for (int i = 0; i < a.size(); i++) {
        for (int j = 0; j < b.size(); i++) {
            if (i == j) {
                x = a.at(index);  // no exception generate by this
                index += 1;
                break;
            }
        }
    }
    return x;
}
