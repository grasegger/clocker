// @generated automatically by Diesel CLI.

diesel::table! {
    blocked (id) {
        id -> Integer,
        date -> Timestamp,
        reason -> Text,
        hours -> Float,
    }
}

diesel::table! {
    entries (id) {
        id -> Integer,
        clock_in -> Timestamp,
        clock_out -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hours (id) {
        id -> Integer,
        start_date -> Timestamp,
        stop_date -> Timestamp,
        mon -> Float,
        tue -> Float,
        wed -> Float,
        thu -> Float,
        fri -> Float,
        sat -> Float,
        sun -> Float,
    }
}

diesel::allow_tables_to_appear_in_same_query!(blocked, entries, hours,);
