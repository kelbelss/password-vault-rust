mod password_entry;

use crate::password_entry::prompt;
use crate::password_entry::read_passwords_from_file;
use crate::password_entry::ServiceInfo;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
  _____                                    _  __      __         _ _   
 |  __ \                                  | | \ \    / /        | | |  
 | |__) __ _ ___ _____      _____  _ __ __| |  \ \  / __ _ _   _| | |_ 
 |  ___/ _` / __/ __\ \ /\ / / _ \| '__/ _` |   \ \/ / _` | | | | | __|
 | |  | (_| \__ \__ \\ V  V | (_) | | | (_| |    \  | (_| | |_| | | |_ 
 |_|   \__,_|___|___/ \_/\_/ \___/|_|  \__,_|     \/ \__,_|\__,_|_|\__|
    "#;

    println!("{ascii}");
    loop {
        println!("Password Vault Menu:");
        println!("1. Add New Password");
        println!("2. List Passwords");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprint!("Error printing passwords:{}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "{}
                    - Username: {}
                    - Password: {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprint!("Error printing passwords:{}", err);
                    Vec::new()
                });
                let search = prompt("Service name: ").to_lowercase();
                for item in &services {
                    if item.service.as_str().to_lowercase() == search.as_str() {
                        println!(
                            "Service: {}
                            - Username: {}
                            - Password: {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please choose between 1 and 4."),
        }
        println!("\n\n");
    }
}
