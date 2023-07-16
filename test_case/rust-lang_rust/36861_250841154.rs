 python
os.path.dirname("/") == "/"
os.path.dirname("/foo") == "/"
os.path.dirname("/foo/") == "/foo"
os.path.dirname("") == ""
os.path.dirname("foo") == ""
os.path.dirname("foo/") == "foo"
os.path.dirname("foo/bar") == "foo"
os.path.dirname("./foo") == "."
