rust
impl<A, B, SqlA, SqlB, DB> Queryable<Cons<SqlA, Cons<SqlB, Nil>>, DB> for (A, B) where
    DB: Backend + HasSqlType<SqlA> + HasSqlType<SqlB>,
    DB: HasSqlType<Cons<SqlA, Cons<SqlB, Nil>>>,
    A: Queryable<SqlA, DB>,
    B: Queryable<SqlB, DB>,
    Cons<A, Cons<B, Nil>>: FromSqlRow<Cons<SqlA, Cons<SqlB, Nil>>, DB>,
{
    type Row = Cons<A, Cons<B, Nil>>;

    fn build(Cons(a, Cons(b, Nil)): Self::Row) -> Self {
        (a, b)
    }
}
