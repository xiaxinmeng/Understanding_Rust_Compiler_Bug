sh
gunzip ~/Downloads/objdump-t-{1,2}.txt.gz
cat ~/Downloads/objdump-t-1.txt | grep _ZN | sed -E 's/^.* +([^ ]+)$/\1/' | sort > sym1
cat ~/Downloads/objdump-t-2.txt | grep _ZN | sed -E 's/^.* +([^ ]+)$/\1/' | sort > sym2
