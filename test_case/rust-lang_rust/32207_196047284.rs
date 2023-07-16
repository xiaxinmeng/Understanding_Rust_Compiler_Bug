
$ python3
Python 3.5.1 (default, Mar 13 2016, 13:29:57) 
[GCC 4.2.1 Compatible Apple LLVM 7.0.2 (clang-700.1.81)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import ipaddress
>>> ip = ipaddress.ip_address('2001:db8::')
>>> ip.is_global
False
>>> ip.is_private
True
