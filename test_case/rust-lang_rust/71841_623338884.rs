
# Python 2.7 deprecation in favor of Python 3.x
# has led to inconsistencies among distributions
# that are still unresolved
# regarding the name of the binary `python` file.
# For instance, some distributions stick to using
# `python3` vs. `python2`,
# while others don't have those
# and only refer to `python`.

# As per #71841, `x.py` shebang line refers to `python`.
# Should this not work on your platform when you run `x.py`,
# the easiest solution is to call `your_python x.py` instead.
