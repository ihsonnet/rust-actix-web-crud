// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        href -> Varchar,
        description -> Varchar,
        name -> Varchar,
        base_type -> Varchar,
        schema_location -> Varchar,
        a_type -> Varchar,
        referred_type -> Varchar,
        employee_id -> Int4,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

diesel::joinable!(accounts -> employees (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    employees,
);
