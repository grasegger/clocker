// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        clock_in -> Timestamp,
        clock_out -> Nullable<Timestamp>,
    }
}
