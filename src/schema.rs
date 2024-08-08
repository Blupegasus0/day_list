// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        name -> Varchar,
        completed -> Bool,
        notes -> Nullable<Text>,
        date -> Date,
    }
}
