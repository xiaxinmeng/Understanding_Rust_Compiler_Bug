sh
# extract offset of the boot partition
echo -e 'unit b\np' | parted disk.img
 2      32768B       236033023B   236000256B   ext2            untitled
# mount the partition
sudo mount -o loop,ro,offset=32768 -text2 disk.img somedir
# copy out the kernel/initrd and unmount.
