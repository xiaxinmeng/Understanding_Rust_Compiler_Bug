 rust

#![feature(macro_rules)]

pub trait ToSql {
    fn to_sql(&self) -> String;
}

#[deriving(Clone)]
pub enum ExprValue<T> {
    ExpressionValue,
    DefaultValue
}

#[allow(dead_code)]
#[deriving(Clone)]
pub enum Insert<T, V, M> {
    InsertDefaultValues,
    InsertValues(Vec<V>)
}

#[allow(dead_code)]
#[deriving(Clone)]
pub struct InsertQuery<T, V, M> {
    pub values: Insert<T, V, M>
}

impl ToSql for ()  {
    fn to_sql(&self) -> String {
        "DEFAULT VALUES".to_string()
    }
}

impl<T: Clone, V: ToSql, M: Clone> ToSql for Insert<T, V, M> {
    fn to_sql(&self) -> String {
        match self {
            &InsertDefaultValues => {
                "DEFAULT VALUES".to_string()
            },
            &InsertValues(ref rows) => {
                let val: Vec<&V> = rows.iter().collect();
                "".to_string()
            }

        }
    }
}

impl<T: Clone, V: Clone+ToSql, M: Clone> ToSql for InsertQuery<T, V, M> {
    fn to_sql(&self) -> String {
        // ..
        format!("{}", self.values.to_sql())
    }
}
