#[derive(Debug, Clone)]
pub struct Classies {
    name: String,
}

impl Classies {
    pub fn new(name: String) -> Classies {
        Classies { name }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
