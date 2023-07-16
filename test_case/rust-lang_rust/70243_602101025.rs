
OutputTypeParameterMismatch(
  Binder(
    <[closure@server/model/core/select.rs:357:26: 357:35] as std::ops::FnMut<(
      <column::Column<select_tests::test::Table, types::I32Type> as selection::Selection<'_, select_tests::test::Table>>::Data,
    )>>
  ),
  Binder(
    <[closure@server/model/core/select.rs:357:26: 357:35] as std::ops::FnMut<(
      i32,
    )>>
  ),
  Sorts(
    ExpectedFound {
      expected: i32,
      found: <column::Column<select_tests::test::Table, types::I32Type> as selection::Selection<'_, select_tests::test::Table>>::Data
    }
  )
)
