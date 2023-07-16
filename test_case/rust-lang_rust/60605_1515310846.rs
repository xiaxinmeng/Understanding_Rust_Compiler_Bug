sh
echo 'abi.cp15_barrier=2' > /etc/sysctl.d/99-cp15_barrier.conf
sysctl -p /etc/sysctl.d/99-cp15_barrier.conf
