// @generated automatically by Diesel CLI.

diesel::table! {
    latlong (id) {
        id -> Varchar,
        longitude -> Varchar,
        latitude -> Varchar,
    }
}

diesel::table! {
    student (account) {
        account -> Varchar,
        psd -> Varchar,
        points -> Int4,
    }
}
