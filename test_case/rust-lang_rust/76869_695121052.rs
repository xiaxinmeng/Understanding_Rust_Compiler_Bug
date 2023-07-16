python
for i, line in enumerate(open("mycode.rs")):
    if any(ord(c) >= 127 for c in line):
        print(i)
