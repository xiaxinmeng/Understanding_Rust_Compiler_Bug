c++
#include <array>
#include <charconv>
#include <iostream>
#include <string_view>
#include <system_error>

int main()
{
    std::array<double, 7> values = { 0.5, 0.25, 0.75, 0.125, 0.35, 0.155, 0.165 };
    std::array<char, 10> buffer{ 0 };

    for (double value : values)
    {
        if (auto [ptr, ec] = std::to_chars(buffer.data(), buffer.data() + buffer.size(), value, std::chars_format::fixed); ec == std::errc{})
        {
            std::cout << std::string_view(buffer.data(), ptr - buffer.data());
        }
        else
        {
            std::cout << "(FAIL)";
        }

        for (int precision = 0; precision < 3; ++precision)
        {
            if (auto [ptr, ec] = std::to_chars(buffer.data(), buffer.data() + buffer.size(), value, std::chars_format::fixed, precision); ec == std::errc{})
            {
                std::cout << '\t' << std::string_view(buffer.data(), ptr - buffer.data());
            }
            else
            {
                std::cout << "\t(FAIL)";
            }
        }
        std::cout << '\n';
    }
}
