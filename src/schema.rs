// @generated automatically by Diesel CLI.

diesel::table! {
    blocked (id) {
        id -> Integer,
        date -> Timestamp,
        reason -> Text,
        hours -> Integer,
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
        beginning_with -> Timestamp,
        hours_per_week -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blocked,
    entries,
    hours,
);
