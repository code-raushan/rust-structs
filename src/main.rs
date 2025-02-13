struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let u1 = User {
        email: String::from("raushan@email.com"),
        username: String::from("raushan"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", u1.email);
    println!("{}", u1.username);
    println!("{}", u1.active);
    println!("{}", u1.sign_in_count);
}
