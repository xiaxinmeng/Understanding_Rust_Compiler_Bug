
spawning command ./loop-condition-eval.exe 
spawn [open ...]

0
exp11 file10
Executed ./loop-condition-eval.exe, status 0
0

PASS: rust/execute/torture/loop-condition-eval.rs   -O0  execution test
FAIL: rust/execute/torture/loop-condition-eval.rs   -O0  output pattern test
Output was:
0

Should match:
1
. . .
spawning command ./loop-condition-eval.exe 
spawn [open ...]

0
exp11 file10
Executed ./loop-condition-eval.exe, status 0
0

PASS: rust/execute/torture/loop-condition-eval.rs   -O1  execution test
FAIL: rust/execute/torture/loop-condition-eval.rs   -O1  output pattern test
Output was:
0

Should match:
1
. . .
