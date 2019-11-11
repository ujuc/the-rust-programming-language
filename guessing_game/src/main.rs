use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // 불변 변수
    let tree = 3;
    // 가변 변수
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("immutable: {}", tree);
}
