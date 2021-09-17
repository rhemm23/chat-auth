table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        pass_hash -> Varchar,
        pass_salt -> Varchar,
        created_at -> Timestamptz,
    }
}
