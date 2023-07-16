
$ echo 'int main() { foo x; }' | LC_ALL=en_US.UTF-8 gcc -x c - 2>&1 | od -t x1 -c
0000000 3c 73 74 64 69 6e 3e 3a 20 49 6e 20 66 75 6e 63
          <   s   t   d   i   n   >   :       I   n       f   u   n   c
0000020 74 69 6f 6e 20 e2 80 98 6d 61 69 6e e2 80 99 3a
          t   i   o   n     342 200 230   m   a   i   n 342 200 231   :
0000040 0a 3c 73 74 64 69 6e 3e 3a 31 3a 31 34 3a 20 65
         \n   <   s   t   d   i   n   >   :   1   :   1   4   :       e
0000060 72 72 6f 72 3a 20 75 6e 6b 6e 6f 77 6e 20 74 79
          r   r   o   r   :       u   n   k   n   o   w   n       t   y
0000100 70 65 20 6e 61 6d 65 20 e2 80 98 66 6f 6f e2 80
          p   e       n   a   m   e     342 200 230   f   o   o 342 200
0000120 99 0a
        231  \n
0000122
