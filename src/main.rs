extern crate diesel;

mod configs;
mod db_schemas;
mod models;
mod rabbitmq;
mod services;

use dotenv::dotenv;

// use rabbitmq::consumer::rabbitmq_consumer;
// use rabbitmq::publisher::rabbitmq_publisher;

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

    // Cria aluno novo e persiste na base.
    let aluno = Aluno::new(("Izabela").to_string());
    aluno.create_aluno();

}
