bash
for i in $(seq 20) ; do
  echo 10000 > /proc/sys/kernel/ns_last_pid
  taskset 1 setarch `arch` -R strace -s 4096 -o rustc$i.strace -f ./build.sh
done
