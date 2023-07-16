
error: internal compiler error: broken MIR in DefId(0/0:6 ~ playground[d6cf]::foo[0]) (bb0[0]): equate_inputs_and_outputs: `GenOnce::<usize, usize>==GenOnce::<usize, usize>` failed with `NoSolution`
  --> src/main.rs:18:1
   |
18 | const foo: GenOnce<usize, usize> = const_generator(10, 100);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
