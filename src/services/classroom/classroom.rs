use crate::services::{student::student::Student, classies::classies::Classies};

#[derive(Debug)]
// pub struct Turma<'a, 'b> {
//     nome: String,
//     alunos: Vec<&'a Aluno>,
//     aulas: Vec<&'b Aula>,
// }

pub struct Classroom {
    name: String,
    students: Vec<Student>,
    classies: Vec<Classies>,
}

// impl<'a, 'b> Turma<'a, 'b> {
//     pub fn new(name: String, alunos: Vec<&'a Aluno>, aulas: Vec<&'b Aula>) -> Turma<'a, 'b> {
//         Turma {
//             nome: name,
//             alunos: alunos,
//             aulas: aulas,
//         }
//     }

//     pub fn get_nome(&self) -> &String {
//         &self.nome
//     }

//     pub fn get_alunos(&self) -> &Vec<&Aluno> {
//         &self.alunos
//     }

//     pub fn get_aulas(&self) -> &Vec<&Aula> {
//         &self.aulas
//     }
// }

impl Classroom {
    pub fn new(name: String, students: Vec<Student>, classies: Vec<Classies>) -> Classroom {
        Classroom {
            name: name,
            students: students,
            classies: classies,
        }
    }

    pub fn get_nome(&self) -> &String {
        &self.name
    }

    pub fn get_alunos(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn get_aulas(&self) -> &Vec<Classies> {
        &self.classies
    }
}
