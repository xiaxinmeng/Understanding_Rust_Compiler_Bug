c
typedef struct { unsigned char data[64]; } Big;
typedef struct { unsigned char data; } Small;

void test(Big, Big, Big, Big, Big, Big, Small);

int main() {
    Big big;
    Small small = { 42 };
    test(big, big, big, big, big, big, small);
}
