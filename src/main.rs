struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut u1 = User {
        email: String::from("raushan@email.com"),
        username: String::from("raushan"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", u1.email);
    println!("{}", u1.username);
    println!("{}", u1.active);
    println!("{}", u1.sign_in_count);

    u1.username = String::from("raushank");
    println!("{}", u1.username);

    let u2 = build_user(String::from("r2@email.com"), String::from("r2"));
    println!("{}", u2.email);

    // reusing instance data
    let u3 = User {
        email: String::from("raushan3@gmail.com"),
        username: String::from("raushan3"),
        ..u2
    };

    println!("{}", u3.active);
}

fn build_user(email: String, username: String)-> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}