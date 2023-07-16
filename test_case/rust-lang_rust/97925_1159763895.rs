
open("/proc/self/cgroup", O_RDONLY|O_CLOEXEC) = 3
syscall_332(0, 0, 0, 0xfff, 0, 0x20)    = -1 (errno 38)
fstat(3, {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
lseek(3, 0, SEEK_CUR)                   = 0
read(3, "10:blkio:/docker/0c8c48d5c6b0391"..., 128) = 128
read(3, "41c9f03b116d6ac4ffa7d6a3fbef7c3d", 32) = 32
read(3, "db1fb03\n8:pids:/docker/0c8c48d5c"..., 96) = 96
read(3, "uacct:/docker/0c8c48d5c6b0391423"..., 256) = 256
read(3, "evices:/docker/0c8c48d5c6b039142"..., 512) = 333
read(3, "", 179)                        = 0
close(3)                                = 0
stat("/sys/fs/cgroup/cpu/docker/0c8c48d5c6b03914239a8179941c9f03b116d6ac4ffa7d6a3fbef7c3ddb1fb03", 0x7ffc2c975b10) = -1 ENOENT (No such file or directory)
stat("/sys/fs/cgroup/cpu,cpuacct/docker/0c8c48d5c6b03914239a8179941c9f03b116d6ac4ffa7d6a3fbef7c3ddb1fb03", 0x7ffc2c975b10) = -1 ENOENT (No such file or directory)
open("/proc/self/mountinfo", O_RDONLY|O_CLOEXEC) = 3
read(3, "203 172 0:41 / / rw,relatime mas"..., 8192) = 4084
close(3)                                = 0
stat("/sys/fs/cgroup/cpu,cpuacct/", {st_mode=S_IFDIR|0755, st_size=0, ...}) = 0
open("/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us", O_RDONLY|O_CLOEXEC) = 3
fstat(3, {st_mode=S_IFREG|0644, st_size=0, ...}) = 0
lseek(3, 0, SEEK_CUR)                   = 0
read(3, "300000\n", 20)                 = 7
read(3, "", 13)                         = 0
close(3)                                = 0
open("/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_period_us", O_RDONLY|O_CLOEXEC) = 3
fstat(3, {st_mode=S_IFREG|0644, st_size=0, ...}) = 0
lseek(3, 0, SEEK_CUR)                   = 0
read(3, "100000\n", 20)                 = 7
read(3, "", 13)                         = 0
close(3)                                = 0
sched_getaffinity(0, 128, [0, 1, 2, 3, 4]) = 64
write(1, "Ok(3)\n", 6Ok(3)
)                  = 6
