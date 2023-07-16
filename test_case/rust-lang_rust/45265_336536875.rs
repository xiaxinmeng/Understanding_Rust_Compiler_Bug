
socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 3
ioctl(3, FIONBIO, [1])                  = 0
connect(3, {sa_family=AF_INET, sin_port=htons(80), sin_addr=inet_addr("127.0.0.1")}, 16) = -1 EINPROGRESS (Operation now in progress)
ioctl(3, FIONBIO, [0])                  = 0
clock_gettime(CLOCK_MONOTONIC, {tv_sec=632431, tv_nsec=149012221}) = 0
clock_gettime(CLOCK_MONOTONIC, {tv_sec=632431, tv_nsec=149240049}) = 0
poll([{fd=3, events=POLLOUT}], 1, 2999) = 1 ([{fd=3, revents=POLLOUT|POLLERR|POLLHUP}])
