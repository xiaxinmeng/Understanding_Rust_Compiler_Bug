c++
struct NoCopyOverload {
    std::vector<uint8_t> vector;
    NoCopyOverload& operator=(const NoCopyOverload&) = default;
};
