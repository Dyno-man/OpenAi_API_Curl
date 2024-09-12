use OpenAi_Curl::*;

fn main() {
    let mut system = Message::new(Role::System, String::from("This will be a test run as my assistant"));
    let mut user = Message::new(Role::User, String::from("What is the first 20 digits of pi"));

    let temp = send_message(&mut system, &mut user);
    println!("{}", temp);
}