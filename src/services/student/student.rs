use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    configs::db_config::ConfigDB,
    entities::student_entity::student_entity::students,
    models::student_model::{StudentModel, NewStudent},
};

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
}

impl Student {
    pub fn new(name: String) -> Student {
        Student { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    

    pub fn update_student_by_id(&self, id: i32) {
        let mut config = ConfigDB::new();
        diesel::update(students::table.filter(students::id.eq(id)))
            .set(students::name.eq(self.name.clone()))
            .execute(&mut config.conn)
            .unwrap();
    }

    pub fn delete_student_by_id(&self, id: i32) {
        let mut config = ConfigDB::new();
        diesel::delete(students::table.filter(students::id.eq(id)))
            .execute(&mut config.conn)
            .unwrap();
    }
}

pub fn get_students() -> Result<Vec<StudentModel>, diesel::result::Error> {
    let mut config = ConfigDB::new();
    let new_student = students::table.load(&mut config.conn);
    return new_student;
}

pub fn get_student_by_id(id: i32) -> Result<StudentModel, diesel::result::Error> {
    let mut config = ConfigDB::new();
    let new_student = students::table
        .filter(students::id.eq(id))
        .first(&mut config.conn);
    return new_student;
}

pub fn create_new_student(name: String) {
    let mut config = ConfigDB::new();

    let new_student = NewStudent {
        name: name.clone(),
    };

    diesel::insert_into(students::table)
        .values(new_student)
        .execute(&mut config.conn)
        .unwrap();
}
