bash
ps -AL --no-headers | wc -l
cat /proc/meminfo
ulimit -Sa
ulimit -Ha
cat /proc/sys/kernel/threads-max
cat /proc/sys/vm/max_map_count
cat /proc/sys/kernel/pid_max
systemctl --version
systemctl status user-$UID.slice
cat /proc/sys/vm/overcommit_*
