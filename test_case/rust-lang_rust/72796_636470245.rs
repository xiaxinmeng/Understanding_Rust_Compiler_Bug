
error: internal compiler error: broken MIR in DefId(0:493 ~ rustc_expand[1d72]::base[0]::{{impl}}[9]::new[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [closure@src/librustc_expand/base.rs:805:18: 805:69]
right-hand side has type: [closure@src/librustc_expand/base.rs:805:18: 805:69]
   --> src/librustc_expand/base.rs:805:18
    |
805 |             .map(|features| features.collect::<Vec<Symbol>>().into());
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
