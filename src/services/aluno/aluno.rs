use diesel::RunQueryDsl;

use crate::{
    configs::db_config::ConfigDB, db_schemas::alunos_schema::schema_aluno::alunos_table,
    models::aluno_model::NovoAluno,
};

#[derive(Debug, Clone)]
pub struct Aluno {
    pub nome: String,
}

impl Aluno {
    pub fn new(nome: String) -> Aluno {
        Aluno { nome }
    }

    pub fn get_nome(&self) -> &String {
        &self.nome
    }

    pub fn set_nome(&mut self, nome: String) {
        self.nome = nome;
    }

    pub fn create_aluno(&self) {
        let mut config = ConfigDB::new();

        let novo_aluno = NovoAluno {
            nome: self.nome.clone(),
        };

        diesel::insert_into(alunos_table::table)
            .values(novo_aluno)
            .execute(&mut config.conn)
            .unwrap();
    }
}
