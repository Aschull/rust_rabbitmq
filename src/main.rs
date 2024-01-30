extern crate diesel;

mod configs;
mod entities;
mod models;
mod rabbitmq;
mod services;
mod traits;

use dotenv::dotenv;

// use rabbitmq::consumer::rabbitmq_consumer;
// use rabbitmq::publisher::rabbitmq_publisher;

use services::student::student::Student;
use services::classies::classies::Classies;
use services::classroom::classroom::Classroom;

use crate::services::student::student::{create_new_student, get_student_by_id, get_students};

fn main() {
    dotenv().ok();

    let mut student = Student::new(("Andrews").to_string());
    let classies: Classies = Classies::new("Algoritmos".to_string());
    let classroom: Classroom = Classroom::new("A-1".to_string(), vec![student.clone()], vec![classies.clone()]);

    println!("{:?}", student.get_name());
    println!("{:?}", classies.get_name());
    println!("{:?}", classroom);
    println!("{:?}", classroom.get_nome());
    println!("{:?}", classroom.get_alunos());
    println!("{:?}", classroom.get_aulas());
    student.set_name("Teste".to_string());
    println!("{:?}", student);
    println!("{:?}", classroom);

    // #TODO Criar uma thread para o publisher funcionar enquanto o consumer estiver funcionando

    // let message = "Ola Mundo".to_string();
    // let routing_key = "hello".to_string();

    // Start Consumer
    // let _ = rabbitmq_consumer();

    // Publish Message
    // let _ = rabbitmq_publisher(message, routing_key);

    // Cria aluno novo e persiste na base.

    /*
    CRUD
        let some_student = Student::new(("Bellinha").to_string());

        create_new_student("Renilto".to_string());
        println!("{:?}", get_students());
        println!("{:?}", get_students_by_id(3).unwrap());
        some_student.update_student_by_id(3);
        some_student.delete_student_by_id(1);
     */
    
}
