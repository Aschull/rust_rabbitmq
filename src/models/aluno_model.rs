use crate::db_schemas::alunos_schema::schema_aluno::alunos_table;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = alunos_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AlunoModel {
    pub nome: String,
}

#[derive(Insertable)]
#[diesel(table_name = alunos_table)]
pub struct NovoAluno {
    pub nome: String,
}
