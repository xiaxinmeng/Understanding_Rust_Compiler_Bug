
# mkdir /sys/fs/cgroup/memory/rust
# echo 0 > /sys/fs/cgroup/memory/rust/tasks tasks # to add the current process to the cgroup
# su -l non_root_user
$ exec process_to_test 
