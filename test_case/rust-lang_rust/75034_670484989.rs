bash
PYTHON=`which python3 || which python2 || which python` 2>/dev/null
"${PYTHON}" ./x.py $@
