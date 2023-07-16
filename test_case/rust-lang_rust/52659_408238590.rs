
#include <stdio.h>
#include <stdexcept>

int main()
{
        printf("Hello ...\n");
        try {
                throw std::runtime_error("this is an exception message");
        } catch (const std::exception& ex) {
                fprintf(stderr, "Error: %s\n", ex.what());
        }

        printf("World\n");
        return 0;
}
