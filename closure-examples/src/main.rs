fn main() {
    let mut greeting = String::from("Hello, ");

    let greet = move |name| {
        greeting.push_str(name);
        greeting.push('!');
        println!("{}", greeting);
    };

    greet.clone()("Alice");
    greet.clone()("Bob");
}
