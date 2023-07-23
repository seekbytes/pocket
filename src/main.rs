
use mfa::interpreter::Compile;
use mfa::interpreter::Intrepreter;
use std::io;

fn main() -> io::Result<()>{
    println!("Write an expression to obfuscate it.");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
    let result = Intrepreter::from_source(&user_input);
    println!("Final result: {:?}", result);

    Ok(())
}