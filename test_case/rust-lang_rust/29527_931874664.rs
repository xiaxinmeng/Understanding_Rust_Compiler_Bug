
$ diff <(echo 'int main() {}' | g++ -static-libgcc --verbose -xc++ - 2>&1 | tr ' ' '\n') \
       <(echo 'int main() {}' | g++ --verbose -xc++ - 2>&1 | tr ' ' '\n')
...
269a270
> -lgcc_s
271d271
< -lgcc_eh
272a273
> -lgcc_s
274d274
< -lgcc_eh
277,278c277,278
...
