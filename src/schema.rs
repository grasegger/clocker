// @generated automatically by Diesel CLI.

diesel::table! {
    in_out (id) {
        id -> Nullable<Integer>,
        clock_in -> Nullable<Text>,
        clock_out -> Nullable<Text>,
    }
}
