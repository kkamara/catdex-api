// @generated automatically by Diesel CLI.

diesel::table! {
    cats (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        image_path -> Varchar,
    }
}
