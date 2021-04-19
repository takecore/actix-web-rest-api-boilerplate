table! {
    companies (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        company_id -> Unsigned<Bigint>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(users -> companies (company_id));

allow_tables_to_appear_in_same_query!(
    companies,
    users,
);
