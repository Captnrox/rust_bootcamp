static mut GLOBAL_ID: u32 = 0;

struct User {
    id: u32,
    user: String,
    mail: String,
    active: bool,
    login_count: u32,
}

impl User {
    fn new(email: String, username: String) -> User {
        let id = Self::increase_id();  

        User {
            id,
            user: username,
            mail: email,
            active: true,
            login_count: 1,
        }
    }

    fn increase_id() -> u32 {
        unsafe {
            GLOBAL_ID += 1;  
            GLOBAL_ID
        }
    }
}


fn main(){
    let user = User::new(String::from("user@example.com"), String::from("john_doe"));
    let user2 = User::new(String::from("user2@example.com"), String::from("johnny_doe"));

    println!("User ID: {}", user.id);
    println!("Username: {}", user.user);

    println!("User ID: {}", user2.id);
    println!("Username: {}", user2.user);
}