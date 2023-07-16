python
from os import listdir
from os.path import isdir, isfile, join

PATHS = ["src/test/ui", "src/test/ui-fulldeps"]

def do_something(path):
    files = [join(path, f) for f in listdir(path)]

    for f in files:
        if isdir(f):
            do_something(f)
            continue
        if not isfile(f) or not f.endswith(".stderr"):
            continue
        x = open(f, "r")
        content = x.read()
        if "error[E" not in content:
            continue
        x = open(f, "w")
        x.write(content.replace("If you want more information on an error, try using",
                                "For more information about an error, try")
                       .replace("If you want more information on this error, try using",
                                "For more information about this error, try"))

for path in PATHS:
    do_something(path)
