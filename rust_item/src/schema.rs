// @generated automatically by Diesel CLI.

diesel::table! {
    student (id) {
        id -> Int4,
        account -> Varchar,
        psd -> Varchar,
        points -> Int4,
    }
}
