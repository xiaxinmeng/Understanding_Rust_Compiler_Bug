
#include <stdio.h>
#include <windows.h>

int main() {
    HMODULE hLibrary = LoadLibrary((LPCWSTR)"test.dll");
    if (hLibrary == NULL) {
        printf("Error while importing DLL: %lu\n", GetLastError());
        return 1;
    }

    return 0;
}
