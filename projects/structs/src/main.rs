struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Coordinate(f64, f64);

fn main() {
    {
        let user1 = User {
            active: true,
            username: String::from("test"),
            email: String::from("test@test"),
            sign_in_count: 1
        };
        let mut user2 = User {
            active: user1.active,
            username: user1.username.clone(),
            email: user1.email.clone(),
            sign_in_count: user1.sign_in_count
        };
        user2.username.push_str("new");
        user2.email.clear();
        user2.email = String::from("test2@test");
    
        println!("u1 {}, {}, {}, {}",
            user1.active, user1.username, user1.email, user1.sign_in_count);
        println!("u2 {}, {}, {}, {}",
            user2.active, user2.username, user2.email, user2.sign_in_count);
    } {
        let user3 = build_user(
            String::from("user3@domain.com"),
            String::from("user3")
        );
        println!("u3 {}, {}, {}, {}",
            user3.active, user3.username, user3.email, user3.sign_in_count);
        let mut user4 = User {
            active: false,
            ..user3
        };
        user4.sign_in_count += 1;
        user4.email.push_str(".com");
        // println!("u3 {}, {}, {}, {}",
        //     user3.active, user3.username, user3.email, user3.sign_in_count);
        println!("u4 {}, {}, {}, {}",
            user4.active, user4.username, user4.email, user4.sign_in_count);
    } {
        let location = Coordinate(1.5, 8.2);
        println!("({}, {})", location.0, location.1);
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
