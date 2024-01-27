
#[macro_use]
extern crate diesel;

mod db_schemas;
mod models;
mod rabbitmq;
mod services;

use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use std::env;
use dotenv::dotenv;

use diesel_migrations::embed_migrations;

// use rabbitmq::consumer::rabbitmq_consumer;
// use rabbitmq::publisher::rabbitmq_publisher;

use crate::db_schemas::alunos_schema::schema_aluno::alunos_table;
use crate::models::aluno_model::{AlunoModel, NovoAluno};

use services::aluno::aluno::Aluno;
use services::aula::aula::Aula;
use services::turma::turma::Turma;



fn main() {

    dotenv().ok();
    
    let mut aluno = Aluno::new(("Andrews").to_string());
    let aula: Aula = Aula::new("Algoritmos".to_string());
    let turma: Turma = Turma::new("A-1".to_string(), vec![aluno.clone()], vec![aula.clone()]);

    println!("{:?}", aluno.get_nome());
    println!("{:?}", aula.get_name());
    println!("{:?}", turma);
    println!("{:?}", turma.get_nome());
    println!("{:?}", turma.get_alunos());
    println!("{:?}", turma.get_aulas());
    aluno.set_nome("Teste".to_string());
    println!("{:?}", aluno);
    println!("{:?}", turma);

    // #TODO Criar uma thread para o publisher funcionar enquanto o consumer estiver funcionando

    // let message = "Ola Mundo".to_string();
    // let routing_key = "hello".to_string();

    // Start Consumer
    // let _ = rabbitmq_consumer();

    // // Publish Message
    // let _ = rabbitmq_publisher(message, routing_key);

    // DataBase
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        embed_migrations!("./migrations/");

        // embedded_migrations::run(&_connection).expect("Erro ao executar migrações");

    let novo_aluno = NovoAluno {
        nome: "Andrews".to_string(),
    };

    diesel::insert_into(alunos_table::table)
        .values(novo_aluno)
        .execute(&mut conn)
        .unwrap();


}
