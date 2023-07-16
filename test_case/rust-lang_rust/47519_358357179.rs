
~ ❯❯❯ python -mtimeit -s "import os; f = open('/dev/zero')" "os.fstat(f.fileno())"
1000000 loops, best of 3: 1.88 usec per loop
~ ❯❯❯ python -mtimeit -s "import os; f = open('/dev/zero')" "os.fstat(f.fileno())"
100000 loops, best of 3: 1.91 usec per loop
~ ❯❯❯ python -mtimeit -s "import os; f = open('/dev/zero')" "os.fstat(f.fileno())"
1000000 loops, best of 3: 1.91 usec per loop
~ ❯❯❯
~ ❯❯❯
~ ❯❯❯
~ ❯❯❯ python -mtimeit -s "import os; f = open('/dev/zero')" "os.stat('/dev/zero')"
100000 loops, best of 3: 3.69 usec per loop
~ ❯❯❯ python -mtimeit -s "import os; f = open('/dev/zero')" "os.stat('/dev/zero')"
100000 loops, best of 3: 3.65 usec per loop
~ ❯❯❯ python -mtimeit -s "import os; f = open('/dev/zero')" "os.stat('/dev/zero')"
100000 loops, best of 3: 3.78 usec per loop
