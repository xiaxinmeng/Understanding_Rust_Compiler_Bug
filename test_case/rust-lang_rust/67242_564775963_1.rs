c++
void foo();
int main() {
    try {
        foo();
    } catch (...) {
        printf("caught exception in catch (...)\n");
        throw;
    }
}
