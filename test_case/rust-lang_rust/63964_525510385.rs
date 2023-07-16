
git checkout 53df91a9b24ad999e7ca896447af6f5f74fe43bc
rm -rf build
x.py build nonexistent/path/to/trigger/cargo/metadata
git checkout 72bfc375356b56933955e07471f91ef97dceaa94
x.py build nonexistent/path/to/trigger/cargo/metadata # fails here w/ unsafe block error
