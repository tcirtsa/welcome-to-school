// @generated automatically by Diesel CLI.
diesel::table! {
    student (account) {
        account -> Varchar,
        psd -> Varchar,
        points -> Int4,
    }
}
