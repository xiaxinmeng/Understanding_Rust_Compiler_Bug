
unsigned __int128 x = 42;

void cas128()
{
    __sync_val_compare_and_swap(&x, 42, 43);
}
