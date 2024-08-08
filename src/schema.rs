// @generated automatically by Diesel CLI.

diesel::table! {
    label (id) {
        id -> Nullable<Integer>,
        name -> Text,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    todo (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        date_created -> Nullable<Text>,
        completed -> Bool,
        due_date -> Nullable<Text>,
        reminder_date -> Nullable<Text>,
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
