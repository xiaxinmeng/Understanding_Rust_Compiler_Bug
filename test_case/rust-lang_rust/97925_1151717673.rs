
root@02b29e5cb9b5:/# cat /proc/self/cgroup
14:name=systemd:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
13:rdma:/
12:pids:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
11:hugetlb:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
10:net_prio:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
9:perf_event:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
8:net_cls:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
7:freezer:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
6:devices:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
5:memory:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
4:blkio:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
3:cpuacct:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
2:cpu:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
1:cpuset:/docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de
0::/

root@02b29e5cb9b5:/tmp/a# cat /proc/self/mountinfo  | grep cpu
1119 1074 0:33 /docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de /sys/fs/cgroup/cpuset ro,nosuid,no
dev,noexec,relatime master:66 - cgroup cpuset rw,cpuset
1120 1074 0:34 /docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de /sys/fs/cgroup/cpu ro,nosuid,nodev
,noexec,relatime master:67 - cgroup cpu rw,cpu
1121 1074 0:35 /docker/02b29e5cb9b58c859327b532d77678965de9b116a516818c9f952fbf491f00de /sys/fs/cgroup/cpuacct ro,nosuid,n
odev,noexec,relatime master:68 - cgroup cpuacct rw,cpuacct

root@02b29e5cb9b5:/# ls -al /sys/fs/cgroup/cpu
total 0
drwxr-xr-x  2 root root   0 Jun  9 23:37 .
dr-xr-xr-x 16 root root 320 Jun  9 23:37 ..
-rw-r--r--  1 root root   0 Jun  9 23:38 cgroup.clone_children
-rw-r--r--  1 root root   0 Jun  9 23:37 cgroup.procs
-rw-r--r--  1 root root   0 Jun  9 23:37 cpu.cfs_period_us
-rw-r--r--  1 root root   0 Jun  9 23:37 cpu.cfs_quota_us
-rw-r--r--  1 root root   0 Jun  9 23:38 cpu.rt_period_us
-rw-r--r--  1 root root   0 Jun  9 23:38 cpu.rt_runtime_us
-rw-r--r--  1 root root   0 Jun  9 23:38 cpu.shares
-r--r--r--  1 root root   0 Jun  9 23:38 cpu.stat
-rw-r--r--  1 root root   0 Jun  9 23:38 notify_on_release
-rw-r--r--  1 root root   0 Jun  9 23:38 tasks

root@02b29e5cb9b5:/# cat /sys/fs/cgroup/cpu/cpu.cfs_quota_us
400000

root@02b29e5cb9b5:/# cat /sys/fs/cgroup/cpu/cpu.cfs_period_us
100000
