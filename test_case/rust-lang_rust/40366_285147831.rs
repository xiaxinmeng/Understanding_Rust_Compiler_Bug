
❯❯❯ echo 'all: ; echo $(MAKEFLAGS)' | make -f /dev/stdin -j8
echo  --jobserver-fds=3,4 -j
--jobserver-fds=3,4 -j
