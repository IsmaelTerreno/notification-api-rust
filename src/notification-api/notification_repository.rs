pub mod notification_repository {
    // use diesel::prelude::*;
    //
    // #[derive(Queryable, Selectable)]
    // #[diesel(table_name = crate::schema::notifications)]
    // #[diesel(check_for_backend(diesel::pg::Pg))]
    // pub struct Notification {
    //     pub id: String,
    //     pub topic: String,
    //     pub body: String,
    //     pub read: bool,
    // }

// use crate::schema::users;
//
// #[derive(Debug, Queryable)]
// pub struct User {
//     pub id: i32,
//     pub name: String,
//     pub password: String,
// }
//
// #[derive(Insertable)]
// #[table_name = "users"]
// pub struct NewUser {
//     pub name: String,
//     pub password: String,
// }

    pub fn hi_now() {
        println!("Hi now.");
    }

    pub fn create() {
        println!("Hi later.")
    }
}