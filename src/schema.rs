// @generated automatically by Diesel CLI.

diesel::table! {
    label (id) {
        id -> Integer,
        name -> Text,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    todo (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        completed -> Bool,
        parent_todo_id -> Nullable<Integer>,
    }
}

diesel::table! {
    todo_label (todo_id, label_id) {
        todo_id -> Nullable<Integer>,
        label_id -> Nullable<Integer>,
    }
}

diesel::joinable!(todo_label -> label (label_id));
diesel::joinable!(todo_label -> todo (todo_id));

diesel::allow_tables_to_appear_in_same_query!(
    label,
    todo,
    todo_label,
);
