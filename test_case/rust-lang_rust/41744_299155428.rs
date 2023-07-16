rust
pub trait ToColumnData {
    fn to_column_data(&self) -> ColumnData;
}
pub trait ToSql : ToColumnData {
    fn to_sql(&self) -> &'static str;
}
