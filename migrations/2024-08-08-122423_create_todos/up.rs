use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_types::{Integer, Varchar, Bool, Text, Date};

table! {
    todos (id) {
        id -> Integer,
        name -> Varchar,
        completed -> Bool,
        notes -> Nullable<Text>,
        date -> Date,
    }
}
