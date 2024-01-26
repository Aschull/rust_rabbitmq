#[derive(Debug, Clone)]
pub struct Aula {
    name: String,
}

impl Aula {
    pub fn new(name: String) -> Aula {
        Aula { name }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
