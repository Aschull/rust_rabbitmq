// use diesel::{prelude::*, query_dsl::methods::LoadQuery};
// use crate::{configs::db_config::ConfigDB, db_schemas::alunos_schema::schema_aluno::alunos_table, models::aluno_model::AlunoModel};

// impl LoadQuery<'_, ConfigDB, AlunoModel> for alunos_table::table {
//     let mut conn = ConfigDB::new();
//     fn internal_load(conn: conn.conn) -> Result<Vec<AlunoModel>, diesel::result::Error> {
//         // Construa e execute a consulta para carregar os dados da tabela para o tipo AlunoModel
//         Self.load(conn)
//     }

//     type RowIter<'conn>
//     where
//         ConfigDB: 'conn;
// }
