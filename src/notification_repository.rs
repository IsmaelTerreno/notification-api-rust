pub fn find_by_user_id(user_id: &str) -> String {
    println!("User id: {:?}", user_id);
    user_id.to_owned()
}

pub fn update() {
    println!("Hi now.");
}

pub fn create() {
    println!("Hi later.")
}
