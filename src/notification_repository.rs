pub mod notification_repository {
    use diesel::prelude::*;
    use uuid::Uuid;

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::notification)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Notification {
        pub id: Uuid,
        pub topic: String,
        pub body: String,
        pub read: bool,
    }

    pub fn update() {
        println!("Hi now.");
    }

    pub fn create() {
        println!("Hi later.")
    }
}