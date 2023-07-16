c++
> #include <cstdio>
> #include <cstdint>
> 
> int main() {
>     alignas(64) uint8_t bytes[33] = {6, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 4};
>     std::printf("align of bytes %zu\n", alignof(bytes));
>     uint64_t* reader = reinterpret_cast<uint64_t*>(bytes);
>     std::printf("%llu, %llu, %llu, %llu\n", reader[0], reader[1], reader[2], reader[3]);
>     return 0;
> }
> 