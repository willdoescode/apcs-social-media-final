table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        author -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        bio -> Nullable<Varchar>,
        is_admin -> Bool,
        password -> Varchar,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(posts, users,);
