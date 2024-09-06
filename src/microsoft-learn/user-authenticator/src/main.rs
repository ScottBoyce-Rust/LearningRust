use user_authenticator::authentication;

fn main() {
    let mut user = authentication::User::new("Scott", "myPassword");
    println!("The username is: {}", user.get_username());
    user.set_password("newPasswordHere");
}
