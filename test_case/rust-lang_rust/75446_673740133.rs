bash
mkdir direct overlay upper lower work
mount -t overlay overlay -o lowerdir=lower,upperdir=upper,workdir=work overlay
echo "foo" > lower/IN
strace -ffe openat,copy_file_range ./copy.rs
