plain
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:643:1
    |
643 | / iter_n! {
644 | |     #[stable(feature = "rust1", since = "1.0.0")]
645 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
646 | |     /// An iterator over subslices separated by elements that match a predicate
663 | |     fn max_items;
664 | | }
    | |_- in this macro invocation
    |
    |
    = note: `-D missing-docs` implied by `-D warnings`
error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:666:1
    |
666 | / iter_n! {
667 | |     #[stable(feature = "rust1", since = "1.0.0")]
668 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
669 | |     /// An iterator over subslices separated by elements that match a
687 | |     fn max_items;
688 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:690:1
    |
690 | / iter_n! {
691 | |     #[stable(feature = "rust1", since = "1.0.0")]
692 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
693 | |     /// An iterator over subslices separated by elements that match a predicate
710 | |     fn max_items;
711 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:713:1
    |
713 | / iter_n! {
714 | |     #[stable(feature = "rust1", since = "1.0.0")]
715 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
716 | |     /// An iterator over subslices separated by elements that match a
734 | |     fn max_items;
735 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | /  macro_rules! iter_n {
752 | |      (
753 | |          #[$stability:meta]
754 | |          #[fused($fused_stability:meta)]
...   |
823 | |              pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |      };
828 | |  }
    | |__- in this expansion of `iter_n!`
    | |__- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:737:1
    |
737 |  / iter_n! {
738 |  |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
739 |  |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
740 |  |     /// An iterator over subslices separated by elements that match a predicate
760 |  |     fn max_items;
761 |  | }
    |  |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 |  / macro_rules! iter_n {
752 |  |     (
753 |  |         #[$stability:meta]
754 |  |         #[fused($fused_stability:meta)]
...    |
823 |  |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827 |  |     };
828 |  | }
    |  |_- in this expansion of `iter_n!`
    |  |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:763:1
    |
763 | /  iter_n! {
764 | |      #[unstable(feature = "split_inclusive_variants", issue = "none")]
765 | |      #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
766 | |      /// An iterator over subslices separated by elements that match a predicate
786 | |      fn max_items;
787 | |  }
    | |__- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 |  / macro_rules! iter_n {
752 |  |     (
753 |  |         #[$stability:meta]
754 |  |         #[fused($fused_stability:meta)]
...    |
823 |  |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827 |  |     };
828 |  | }
    |  |_- in this expansion of `iter_n!`
    |  |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:789:1
    |
789 | /  iter_n! {
790 | |      #[unstable(feature = "split_inclusive_variants", issue = "none")]
791 | |      #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
792 | |      /// An iterator over subslices separated by elements that match a predicate
812 | |      fn max_items;
813 | |  }
    | |__- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 |  / macro_rules! iter_n {
752 |  |     (
753 |  |         #[$stability:meta]
754 |  |         #[fused($fused_stability:meta)]
...    |
823 |  |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827 |  |     };
828 |  | }
    |  |_- in this expansion of `iter_n!`
    |  |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:815:1
    |
815 | /  iter_n! {
816 | |      #[unstable(feature = "split_inclusive_variants", issue = "none")]
817 | |      #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
818 | |      /// An iterator over subslices separated by elements that match a predicate
838 | |      fn max_items;
839 | |  }
    | |__- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:841:1
    |
841 | / iter_n! {
842 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
843 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
844 | |     /// An iterator over subslices separated by elements that match a predicate
864 | |     fn max_items;
865 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:867:1
    |
867 | / iter_n! {
868 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
869 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
870 | |     /// An iterator over subslices separated by elements that match a predicate
890 | |     fn max_items;
891 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:893:1
    |
893 | / iter_n! {
894 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
895 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
896 | |     /// An iterator over subslices separated by elements that match a predicate
916 | |     fn max_items;
917 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:919:1
    |
919 | / iter_n! {
920 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
921 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
922 | |     /// An iterator over subslices separated by elements that match a predicate
942 | |     fn max_items;
943 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:850:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
850  | |             pub fn as_str(&self) -> &'a str {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1016 | / generate_n_iterators! {
1017 | |     forward:
1018 | |         #[stable(feature = "rust1", since = "1.0.0")]
1019 | |         #[fused(stable(feature = "fused", since = "1.26.0"))]
1043 | |         fn as_str;
1044 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:858:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
858  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1016 | / generate_n_iterators! {
1017 | |     forward:
1018 | |         #[stable(feature = "rust1", since = "1.0.0")]
1019 | |         #[fused(stable(feature = "fused", since = "1.26.0"))]
1043 | |         fn as_str;
1044 | | }
     | |_- in this macro invocation

---
424  | |     {
425  | |         // Forward iterator
426  | |         forward:
...    |
534  | |             pub fn as_str(&self) -> &'a str {
...    |
590  | |     } => {}
591  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1054 | / generate_pattern_iterators! {
1055 | |     forward:
1056 | |         #[stable(feature = "split_inclusive", since = "1.51.0")]
1057 | |         #[fused(stable(feature = "split_inclusive", since = "1.51.0"))]
1092 | |     delegate double ended;
1093 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:850:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
850  | |             pub fn as_str(&self) -> &'a str {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1095 | / generate_n_iterators! {
1096 | |     forward:
1097 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1098 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1122 | |         fn as_str;
1123 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:858:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
858  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1095 | / generate_n_iterators! {
1096 | |     forward:
1097 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1098 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1122 | |         fn as_str;
1123 | | }
     | |_- in this macro invocation

---
424  | |     {
425  | |         // Forward iterator
426  | |         forward:
...    |
534  | |             pub fn as_str(&self) -> &'a str {
...    |
590  | |     } => {}
591  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1133 | / generate_pattern_iterators! {
1134 | |     forward:
1135 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1136 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1172 | |     delegate double ended;
1173 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:850:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
850  | |             pub fn as_str(&self) -> &'a str {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1175 | / generate_n_iterators! {
1176 | |     forward:
1177 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1178 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1202 | |         fn as_str;
1203 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:858:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
858  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1175 | / generate_n_iterators! {
1176 | |     forward:
1177 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1178 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1202 | |         fn as_str;
1203 | | }
     | |_- in this macro invocation

---
424  | |     {
425  | |         // Forward iterator
426  | |         forward:
...    |
485  | |             pub fn as_str(&self) -> &'a str {
...    |
590  | |     } => {}
591  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1213 | / generate_pattern_iterators! {
1214 | |     forward:
1215 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1216 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1238 | |     delegate double ended;
1239 | | }
     | |_- in this macro invocation

---
424  | |     {
425  | |         // Forward iterator
426  | |         forward:
...    |
534  | |             pub fn as_str(&self) -> &'a str {
...    |
590  | |     } => {}
591  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1213 | / generate_pattern_iterators! {
1214 | |     forward:
1215 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1216 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1238 | |     delegate double ended;
1239 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:858:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
858  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1241 | / generate_n_iterators! {
1242 | |     forward:
1243 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1244 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1262 | |         fn as_str;
1263 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:858:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
858  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1327 | / generate_n_iterators! {
1328 | |     forward:
1329 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1330 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1348 | |         fn as_str;
1349 | | }
     | |_- in this macro invocation

---
424  | |     {
425  | |         // Forward iterator
426  | |         forward:
...    |
485  | |             pub fn as_str(&self) -> &'a str {
...    |
590  | |     } => {}
591  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1360 | / generate_pattern_iterators! {
1361 | |     forward:
1362 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1363 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1385 | |     delegate double ended;
1386 | | }
     | |_- in this macro invocation

---
424  | |     {
425  | |         // Forward iterator
426  | |         forward:
...    |
534  | |             pub fn as_str(&self) -> &'a str {
...    |
590  | |     } => {}
591  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1360 | / generate_pattern_iterators! {
1361 | |     forward:
1362 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1363 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1385 | |     delegate double ended;
1386 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:858:13
     |
772  | / macro_rules! generate_n_iterators {
773  | |     (
774  | |         forward:
775  | |             #[$forward_stability_attribute:meta]
...    |
858  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
952  | |     }
953  | | }
953  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1388 | / generate_n_iterators! {
1389 | |     forward:
1390 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1391 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1409 | |         fn as_str;
1410 | | }
     | |_- in this macro invocation

