 bash
i=0;
while [[ $i -lt $j ]]; do
    echo '        FnPtr::new(metaloadfn("'"function"'", &[])),';
    i=$[i+1]
done >> test-slow.rs
sed -i s/LEN/$j/ test-slow.rs
echo ']}}' >> test-slow.rs
