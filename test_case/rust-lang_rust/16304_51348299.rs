
/tmp/bar  
❯ vim hello.rs

/tmp/bar  8s
❯ ls
bin/  hello.rs

/tmp/bar  
❯ rustc -o bin/hello hello.rs

/tmp/bar  
❯ ls bin 
hello*

/tmp/bar  
❯ ./bin/hello 
Hello world
