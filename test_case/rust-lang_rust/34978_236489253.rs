
for i in *; do echo $i; objdump -r $i | grep "RELX"; done
