
   Compiling differential-dataflow v0.0.3
error: no associated item named `new` found for type `timely_sort::Sorter<_>` in the current scope
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\differential-dataflow-0.0.3\src\operators\arrange.rs:122:26
    |
122 |         let mut sorter = LSBRadixSorter::new();
    |                          ^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
    = help: candidate #1: `use timely_sort::RadixSorterBase;`

error: no associated item named `new` found for type `timely_sort::Sorter<_>` in the current scope
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\differential-dataflow-0.0.3\src\operators\arrange.rs:222:26
    |
222 |         let mut sorter = LSBRadixSorter::new();
    |                          ^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
    = help: candidate #1: `use timely_sort::RadixSorterBase;`

error: no associated item named `new` found for type `timely_sort::Sorter<_>` in the current scope
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\differential-dataflow-0.0.3\src\operators\consolidate.rs:110:70
    |
110 |                 let entry = inputs.entry_or_insert(index.time(), || (LSBRadixSorter::new(), 0, default_threshold));
    |                                                                      ^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
    = help: candidate #1: `use timely_sort::RadixSorterBase;`
