table! {
    apps (id) {
        id -> Int4,
        name -> Varchar,
        domain -> Varchar,
        environment -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    routes (id) {
        id -> Int4,
        app_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        pattern -> Varchar,
        view_id -> Nullable<Int4>,
    }
}

table! {
    views (id) {
        id -> Int4,
        app_id -> Nullable<Int4>,
        name -> Varchar,
        kind -> Int2,
        content_url -> Nullable<Varchar>,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(routes -> apps (app_id));
joinable!(routes -> views (view_id));
joinable!(views -> apps (app_id));

allow_tables_to_appear_in_same_query!(
    apps,
    routes,
    views,
);
