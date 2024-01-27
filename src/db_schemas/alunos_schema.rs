pub mod schema_aluno {
    diesel::table! {
        alunos_table {
            id -> Int4,
            nome -> Varchar,
        }
    }
}
