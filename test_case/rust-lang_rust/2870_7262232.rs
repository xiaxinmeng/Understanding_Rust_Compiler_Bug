
true;
while [ "$?" = "0" ]; do
    ./x86_64-unknown-linux-gnu/test/coretest.stage1-x86_64-unknown-linux-gnu setenv;
done
