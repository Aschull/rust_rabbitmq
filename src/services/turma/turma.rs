use crate::services::{aluno::aluno::Aluno, aula::aula::Aula};

#[derive(Debug)]
// pub struct Turma<'a, 'b> {
//     nome: String,
//     alunos: Vec<&'a Aluno>,
//     aulas: Vec<&'b Aula>,
// }

pub struct Turma {
    nome: String,
    alunos: Vec<Aluno>,
    aulas: Vec<Aula>,
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

impl Turma {
    pub fn new(name: String, alunos: Vec<Aluno>, aulas: Vec<Aula>) -> Turma {
        Turma {
            nome: name,
            alunos: alunos,
            aulas: aulas,
        }
    }

    pub fn get_nome(&self) -> &String {
        &self.nome
    }

    pub fn get_alunos(&self) -> &Vec<Aluno> {
        &self.alunos
    }

    pub fn get_aulas(&self) -> &Vec<Aula> {
        &self.aulas
    }
}
