python
#!/usr/bin/python
import os

REMOVE_STRING = ":/home/rdodd/packages/emsdk-portable/node/4.1.1_64bit/bin"

def main():
    old_path = os.environ["PATH"]
    new_path = old_path.replace(REMOVE_STRING, "")
    print(new_path)

if __name__ == '__main__':
    main()
