diff
       StorageLive(_2);                 // bb0[2]: scope 3 at src/main.rs:9:9: 9:14
       StorageLive(_3);                 // bb0[3]: scope 3 at src/main.rs:9:17: 11:6
       _3 = move _1;                    // bb0[4]: scope 3 at src/main.rs:9:17: 11:6
+      The line that causes the error.
       _2 = [closure@src/main.rs:9:17: 11:6] { fancy_num: move _3 }; // bb0[5]: scope 3 at src/main.rs:9:17: 11:6
                                         // closure
                                         // └ def_id: DefId(0/1:10 ~ playground[72d2]::main[0]::{{closure}}[0])
