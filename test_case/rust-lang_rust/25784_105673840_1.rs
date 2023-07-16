
howe-and-ser-moving:~ geofft$ python2 -c 'print [line for line in open("/proc/self/status") if "SigIgn" in line][0]'
SigIgn: 0000000001001000

howe-and-ser-moving:~ geofft$ python2 -c 'import subprocess; print subprocess.check_output(["grep", "SigIgn", "/proc/self/status"])'
SigIgn: 0000000001001000

howe-and-ser-moving:~ geofft$ python3 -c 'print([line for line in open("/proc/self/status") if "SigIgn" in line][0])'
SigIgn: 0000000001001000

howe-and-ser-moving:~ geofft$ python3 -c 'import subprocess, sys; sys.stdout.buffer.write(subprocess.check_output(["grep", "SigIgn", "/proc/self/status"]))'
SigIgn: 0000000000000000
