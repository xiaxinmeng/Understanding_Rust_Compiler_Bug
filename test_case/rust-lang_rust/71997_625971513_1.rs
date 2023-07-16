c++
#include <cassert>
#include <cstdint>
#include <vector>
using namespace std;

uint64_t f(vector<uint64_t> slice, size_t start, size_t end)
{
    uint64_t total = 0;
    assert(start < end && start < slice.size() && end <= slice.size());
    if (end > slice.size())
        end = slice.size();
    for (size_t i = start; i < end; i++) {
        total += slice.at(i);
    }
    return total;
}
