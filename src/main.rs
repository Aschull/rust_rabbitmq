mod rabbitmq;
mod service;

use rabbitmq::consumer::rabbitmq_consumer;
use rabbitmq::publisher::rabbitmq_publisher;

use service::aluno::aluno::Aluno;
use service::aula::aula::Aula;
use service::turma::turma::Turma;

fn main() {
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

    let message = "Ola Mundo".to_string();
    let routing_key = "hello".to_string();

    // Start Consumer
    let _ = rabbitmq_consumer();
   

    // Publish Message
    let _ = rabbitmq_publisher(message, routing_key);
}
