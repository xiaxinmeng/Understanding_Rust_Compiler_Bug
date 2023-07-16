 c
int main(int argc, char **argv)
{
    unsigned char  byte = 0;
    unsigned short port = 0x80;

    asm volatile("out %%al, %%dx" :: "a" (byte), "d" (port));
}
