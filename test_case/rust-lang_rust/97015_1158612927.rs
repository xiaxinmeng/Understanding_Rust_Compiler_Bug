
fn foo(cursor: BorrowCursor<'_, '_>) {
    cursor.ensure_init();
   // use the cursor
}
