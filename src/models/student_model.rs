use crate::entities::student_entity::student_entity::students;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone)]
pub struct StudentModel {
    id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent {
    pub name: String,
}
