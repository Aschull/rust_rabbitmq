use crate::db_schemas::alunos_schema::schema_aluno::alunos_table;
// #[derive(Queryable)]
// pub struct GetAluno {
//     pub id: i32,
//     pub nome: String,
//     // Outros campos da tabela Alunos
// }

// #[derive(Insertable)]
// #[diesel(table_name = "alunos_table")]
// pub struct AlunoModel {
//     pub nome: String,
// }

// impl AlunoModel {
//     pub fn new(nome: String) {
//         AlunoModel { nome };
//     }
// }



use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db_schemas::alunos_schema::schema_aluno::alunos_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AlunoModel {
    pub nome: String,
}



#[derive(Insertable)]
#[diesel(table_name = alunos_table)]
pub struct NovoAluno {
    pub nome: String,
}