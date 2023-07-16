
error[E0080]: constant evaluation error
  --> unic/ucd/name/src/name.rs:44:18
   |
44 |             _ => data::NAMES.find(ch).map(|pieces| Name::NR3(pieces)),
   |                  ^^^^^^^^^^^ referenced constant has errors

error[E0080]: constant evaluation error
     --> unic/ucd/name/src/../tables/name_map.rsv:3:24
      |
3     |   CharDataTable::Direct(&[
      |  ________________________^
4     | |     ('\u{20}', &[SPACE]),
5     | |     ('\u{21}', &[EXCLAMATION, MARK]),
6     | |     ('\u{22}', &[QUOTATION, MARK]),
...     |
31526 | |     ('\u{e01ef}', &[VARIATION, SELECTOR_256]),
31527 | | ])
      | |_^ reached the configured maximum execution time

    time: 4.969; rss: 764MB     translation item collection
error: aborting due to 2 previous errors
