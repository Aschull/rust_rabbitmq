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
}
