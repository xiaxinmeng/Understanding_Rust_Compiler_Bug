
PS C:\Users\WDAGUtilityAccount> $env:PATHEXT+=';.PY'
PS C:\Users\WDAGUtilityAccount> cat x.py
#!/usr/bin/env bash

'''':
echo "hello from bash"
exit 0
'''

print("hello from powershell")
PS C:\Users\WDAGUtilityAccount> ./x
Unable to create process using '/usr/bin/env bash "C:\Users\WDAGUtilityAccount\x.py" '
PS C:\Users\WDAGUtilityAccount> py x.py
Unable to create process using '/usr/bin/env bash x.py'
PS C:\Users\WDAGUtilityAccount> py -3 x.py
hello from powershell
