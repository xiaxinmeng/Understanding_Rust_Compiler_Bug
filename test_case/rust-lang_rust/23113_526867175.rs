
rg --multiline --multiline-dotall 'run-pass.*assert' src/test/ui -l | sort > /tmp/with-assert
rg --multiline --multiline-dotall 'run-pass' src/test/ui -l | sort > /tmp/all
diff -U0 /tmp/all /tmp/with-assert | rg -v '@@'
