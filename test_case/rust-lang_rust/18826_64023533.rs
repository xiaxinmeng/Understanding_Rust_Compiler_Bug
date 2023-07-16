
(foo)$ cat a.py    
import colorama
print(colorama.Fore.RED + "Hello")
(foo)$ python a.py | xxd
0000000: 1b5b 3331 6d48 656c 6c6f 0a              .[31mHello.
