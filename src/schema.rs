table! {
    todo_item (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
        list_id -> Int4,
    }
}

table! {
    todo_list (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(todo_item -> todo_list (list_id));

allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
    users,
);
