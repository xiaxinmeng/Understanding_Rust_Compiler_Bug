
---- [debuginfo-gdb+lldb] debuginfo/lexical-scopes-in-block-expression.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 900
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$27 = 10
status: exit code: 0
command: "/nix/store/fqbrwq2gvv97lb0albmfb2x212azzp46-python3-3.7.3/bin/python" "/home/eddy/Projects/rust-1/src/etc/lldb_batchmode.py" "/home/eddy/Projects/rust-1/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression/a" "/home/eddy/Projects/rust-1/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression/lexical-scopes-in-block-expression.debugger.script"
stdout:
------------------------------------------
Hit breakpoint 1.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 30 at lexical-scopes-in-block-expression.rs:421:12, address = 0x000055555555523e, resolved, hit count = 1 
Hit breakpoint 2.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 98 at lexical-scopes-in-block-expression.rs:427:12, address = 0x0000555555555282, resolved, hit count = 1 
Hit breakpoint 3.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 136 at lexical-scopes-in-block-expression.rs:435:4, address = 0x00005555555552a8, resolved, hit count = 1 
Hit breakpoint 4.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 146 at lexical-scopes-in-block-expression.rs:440:8, address = 0x00005555555552b2, resolved, hit count = 1 
Hit breakpoint 5.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 217 at lexical-scopes-in-block-expression.rs:446:8, address = 0x00005555555552f9, resolved, hit count = 1 
Hit breakpoint 6.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 240 at lexical-scopes-in-block-expression.rs:452:4, address = 0x0000555555555310, resolved, hit count = 1 
Hit breakpoint 7.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 250 at lexical-scopes-in-block-expression.rs:458:8, address = 0x000055555555531a, resolved, hit count = 1 
Hit breakpoint 8.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 321 at lexical-scopes-in-block-expression.rs:464:8, address = 0x0000555555555361, resolved, hit count = 1 
Hit breakpoint 9.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 358 at lexical-scopes-in-block-expression.rs:470:4, address = 0x0000555555555386, resolved, hit count = 1 
Hit breakpoint 10.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 368 at lexical-scopes-in-block-expression.rs:475:8, address = 0x0000555555555390, resolved, hit count = 1 
Hit breakpoint 11.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 433 at lexical-scopes-in-block-expression.rs:481:8, address = 0x00005555555553d1, resolved, hit count = 1 
Hit breakpoint 12.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 483 at lexical-scopes-in-block-expression.rs:487:4, address = 0x0000555555555403, resolved, hit count = 1 
Hit breakpoint 13.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 493 at lexical-scopes-in-block-expression.rs:492:8, address = 0x000055555555540d, resolved, hit count = 1 
Hit breakpoint 14.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 558 at lexical-scopes-in-block-expression.rs:498:8, address = 0x000055555555544e, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/home/eddy/Projects/rust-1/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression/lexical-scopes-in-block-expression.debugger.script'.
Target executable is '/home/eddy/Projects/rust-1/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression/a'.
Current working directory is '/home/eddy/Projects/rust-1'
Creating a target for '/home/eddy/Projects/rust-1/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression/a'
settings set auto-confirm true

version
lldb version 9.0.0 (https://github.com/rust-lang/llvm-project.git revision 48818e9f5d0f2d5978a9b43ad1a2e8d0b83f6aa0) clang revision 48818e9f5d0f2d5978a9b43ad1a2e8d0b83f6aa0 llvm revision 48818e9f5d0f2d5978a9b43ad1a2e8d0b83f6aa0 
command script import /home/eddy/Projects/rust-1/./src/etc/lldb_rust_formatters.py
type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
type category enable Rust

breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 421
Breakpoint 1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 30 at lexical-scopes-in-block-expression.rs:421:12, address = 0x000000000000123e 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 427
Breakpoint 2: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 98 at lexical-scopes-in-block-expression.rs:427:12, address = 0x0000000000001282 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 435
Breakpoint 3: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 136 at lexical-scopes-in-block-expression.rs:435:4, address = 0x00000000000012a8 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 440
Breakpoint 4: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 146 at lexical-scopes-in-block-expression.rs:440:8, address = 0x00000000000012b2 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 446
Breakpoint 5: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 217 at lexical-scopes-in-block-expression.rs:446:8, address = 0x00000000000012f9 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 452
Breakpoint 6: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 240 at lexical-scopes-in-block-expression.rs:452:4, address = 0x0000000000001310 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 458
Breakpoint 7: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 250 at lexical-scopes-in-block-expression.rs:458:8, address = 0x000000000000131a 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 464
Breakpoint 8: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 321 at lexical-scopes-in-block-expression.rs:464:8, address = 0x0000000000001361 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 470
Breakpoint 9: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 358 at lexical-scopes-in-block-expression.rs:470:4, address = 0x0000000000001386 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 475
Breakpoint 10: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 368 at lexical-scopes-in-block-expression.rs:475:8, address = 0x0000000000001390 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 481
Breakpoint 11: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 433 at lexical-scopes-in-block-expression.rs:481:8, address = 0x00000000000013d1 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 487
Breakpoint 12: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 483 at lexical-scopes-in-block-expression.rs:487:4, address = 0x0000000000001403 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 492
Breakpoint 13: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 493 at lexical-scopes-in-block-expression.rs:492:8, address = 0x000000000000140d 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 498
Breakpoint 14: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 558 at lexical-scopes-in-block-expression.rs:498:8, address = 0x000000000000144e 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 504
Breakpoint 15: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1223 at lexical-scopes-in-block-expression.rs:504:4, address = 0x00000000000016e7 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 510
Breakpoint 16: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 631 at lexical-scopes-in-block-expression.rs:510:8, address = 0x0000000000001497 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 516
Breakpoint 17: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 696 at lexical-scopes-in-block-expression.rs:516:8, address = 0x00000000000014d8 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 522
Breakpoint 18: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 722 at lexical-scopes-in-block-expression.rs:522:4, address = 0x00000000000014f2 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 527
Breakpoint 19: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 732 at lexical-scopes-in-block-expression.rs:527:8, address = 0x00000000000014fc 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 533
Breakpoint 20: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 797 at lexical-scopes-in-block-expression.rs:533:8, address = 0x000000000000153d 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 539
Breakpoint 21: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 926 at lexical-scopes-in-block-expression.rs:539:4, address = 0x00000000000015be 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 545
Breakpoint 22: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1365 at lexical-scopes-in-block-expression.rs:545:8, address = 0x0000000000001775 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 551
Breakpoint 23: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1029 at lexical-scopes-in-block-expression.rs:551:8, address = 0x0000000000001625 
breakpoint set --file 'lexical-scopes-in-block-expression.rs' --line 557
Breakpoint 24: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1069 at lexical-scopes-in-block-expression.rs:557:4, address = 0x000000000000164d 
run
Process 90010 stopped * thread #1, name = 'a', stop reason = breakpoint 1.1 frame #0: 0x000055555555523e a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 at lexical-scopes-in-block-expression.rs:421:12 418 // surrounded by struct expression 419 let point = Point { 420 x: { -> 421 zzz(); // #break ^ 422 sentinel(); 423 424 let val = ten + 1; Process 90010 launched: '/home/eddy/Projects/rust-1/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression/a' (x86_64) 
print val
(int) $0 = -1 
print ten
(long) $1 = 10 
continue
print val
(long) $2 = 11 
print ten
(long) $3 = 10 
continue
print val
(int) $4 = -1 
print ten
(long) $5 = 10 
continue
print val
(int) $6 = -1 
print ten
(long) $7 = 10 
continue
print val
(long) $8 = 12 
print ten
(long) $9 = 10 
continue
print val
(int) $10 = -1 
print ten
(long) $11 = 10 
continue
print val
(int) $12 = -1 
print ten
(long) $13 = 10 
continue
print val
(long) $14 = 13 
print ten
(long) $15 = 10 
continue
print val
(int) $16 = -1 
print ten
(long) $17 = 10 
continue
print val
(int) $18 = -1 
print ten
(long) $19 = 10 
continue
print val
(long) $20 = 14 
print ten
(long) $21 = 10 
continue
print val
(int) $22 = -1 
print ten
(long) $23 = 10 
continue
print val
(int) $24 = -1 
print ten
(long) $25 = 10 
continue
print val
(long) $26 = 15 
print ten
(long) $27 = 10 Hit breakpoint 15.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1223 at lexical-scopes-in-block-expression.rs:504:4, address = 0x00005555555556e7, resolved, hit count = 1 
Hit breakpoint 16.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 631 at lexical-scopes-in-block-expression.rs:510:8, address = 0x0000555555555497, resolved, hit count = 1 
Hit breakpoint 17.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 696 at lexical-scopes-in-block-expression.rs:516:8, address = 0x00005555555554d8, resolved, hit count = 1 
Hit breakpoint 18.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 722 at lexical-scopes-in-block-expression.rs:522:4, address = 0x00005555555554f2, resolved, hit count = 1 
Hit breakpoint 19.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 732 at lexical-scopes-in-block-expression.rs:527:8, address = 0x00005555555554fc, resolved, hit count = 1 
Hit breakpoint 20.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 797 at lexical-scopes-in-block-expression.rs:533:8, address = 0x000055555555553d, resolved, hit count = 1 
Hit breakpoint 21.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 926 at lexical-scopes-in-block-expression.rs:539:4, address = 0x00005555555555be, resolved, hit count = 1 
Hit breakpoint 22.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1365 at lexical-scopes-in-block-expression.rs:545:8, address = 0x0000555555555775, resolved, hit count = 1 
Hit breakpoint 23.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1029 at lexical-scopes-in-block-expression.rs:551:8, address = 0x0000555555555625, resolved, hit count = 1 
Hit breakpoint 24.1: where = a`lexical_scopes_in_block_expression::main::hcdd5c3caa9166e73 + 1069 at lexical-scopes-in-block-expression.rs:557:4, address = 0x000055555555564d, resolved, hit count = 1 

continue
print val
(int) $28 = -1 
print ten
(long) $29 = 10 
continue
print val
(int) $30 = -1 
print ten
(long) $31 = 10 
continue
print val
(long) $32 = 16 
print ten
(long) $33 = 10 
continue
print val
(int) $34 = -1 
print ten
(long) $35 = 10 
continue
print val
(int) $36 = -1 
print ten
(long) $37 = 10 
continue
print val
(long) $38 = 17 
print ten
(long) $39 = 10 
continue
print val
(int) $40 = -1 
print ten
(long) $41 = 10 
continue
print val
(int) $42 = -1 
print ten
(long) $43 = 10 
continue
print val
(long) $44 = 18 
print ten
(long) $45 = 10 
continue
print val
(int) $46 = -1 
print ten
(long) $47 = 10 
continue
quit
None

------------------------------------------
stderr:
------------------------------------------
/home/eddy/Projects/rust-1/src/etc/lldb_batchmode.py:139: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = time.clock()
/home/eddy/Projects/rust-1/src/etc/lldb_batchmode.py:143: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while time.clock() < watchdog_max_time:

------------------------------------------
