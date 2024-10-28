mod normal_print;
use std::process::Command;

fn main() {
    normal_print::printf("Enter url: ");
    let mut text:String = String::new();
    std::io::stdin().read_line(&mut text).unwrap();

    text = text.strip_suffix('\n').unwrap().to_string();
    text = text.strip_suffix('\r').unwrap().to_string();

    let vec_text = text.chars().collect::<Vec<char>>();
    let mut vec_pointers:Vec<char> = Vec::new();

    for i in vec_text{
        if i.is_ascii(){
            vec_pointers.push('-');
        } else {
            vec_pointers.push('^');
        }
        print!("{} ", i);
    }
    println!();
    for letter in vec_pointers{
        if letter == '^'{
            print!("{} ", letter);
        }else {
            print!("  ");
        }
    }
    println!();
    println!("Where: ^, means it's not ascii symbols.");

    Command::new("cmd")
        .args(["/C","pause"])
        .spawn()
        .expect("failed to execute process");
}
