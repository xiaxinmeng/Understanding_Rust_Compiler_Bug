`bash
# launch in travis as 'pathto/script.sh &'
while `sleep 30`
do
top -ibn 1 | head -n4 | tr "\n" " " | tee -a /tmp/top.log
echo "" | tee -a /tmp/top.log
done
