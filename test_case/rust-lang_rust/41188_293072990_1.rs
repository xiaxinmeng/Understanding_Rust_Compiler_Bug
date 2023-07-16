
$ echo "#### Build failed; Disk usage after running script:"; df -h; du . | sort -nr | head -n100
    
#### Build failed; Disk usage after running script:
Filesystem      Size   Used  Avail Capacity  iused    ifree %iused  Mounted on
/dev/disk0s2   149Gi   71Gi   78Gi    48% 18580816 20530880   48%   /
devfs          176Ki  176Ki    0Bi   100%      608        0  100%   /dev
map -hosts       0Bi    0Bi    0Bi   100%        0        0  100%   /net
map auto_home    0Bi    0Bi    0Bi   100%        0        0  100%   /home
[...]
