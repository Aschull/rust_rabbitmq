pub mod student_entity {
    diesel::table! {
        students {
            id -> Int4,
            name -> Varchar,
        }
    }
}
