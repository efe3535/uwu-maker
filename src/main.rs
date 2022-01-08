use std::env;

fn main() {
    let letters: [&str; 5] = ["a", "e", "i", "o" "u"];
    let mut uwud: String = "".to_string();  
    let mut args: Vec<String> = env::args().collect();
    let progname: &str = &args.to_vec()[0];
    args.remove(0);
    
    if args.len() >= 1 && args[0] != "--help" {
        for mut arg in args {
            if letters.contains(&&arg.chars().last().unwrap().to_string().as_str()) {
                arg.push_str("wu ");
                uwud.push_str(arg.as_str());
            } else {
                arg.push_str("uwu ");
                uwud.push_str(arg.as_str());
            }
        }
    } else {
        println!("usage: {} <string to make uwu>",progname);
    }
    
    println!("{}", uwud);
}

