
howe-and-ser-moving:~ geofft$ ruby -e 'open("/proc/self/status").each_line { |line| puts line if line.include? "SigIgn" }'
SigIgn: 0000000000001000
howe-and-ser-moving:~ geofft$ ruby -e 'puts `grep SigIgn /proc/self/status`'
SigIgn: 0000000000000000
