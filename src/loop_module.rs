use std::collections::HashMap;
use prettytable::{Cell, Row, Table};
use prettytable::row;
use std::io::{self, Write};
use nix::libc;
use whoami;
use colored::Colorize;
use std::env;
use std::io::BufReader;
use std::fs::{File, remove_file};
use csv::Reader;
use crate::csv_handle;
use crate::encryption::{self, add_space};
use std::io::Result;

struct User {
    app_name: String,
    username: String,
    password: String,
}

fn start() {
    println!("\x1B[31m{}\x1B[0m", format!("__________                                  .___                                                           "));
    println!("\x1B[31m{}\x1B[0m", format!("\\______   \\_____    ______ ________  _  ____| _/          _____ _____    ____ _____     ____   ___________ "));
    println!("\x1B[31m{}\x1B[0m", format!(" |     ___/\\__  \\  /  ___//  ___/\\ \\/ \\/ / __ |  ______  /     \\\\__  \\  /    \\\\__  \\   / ___\\_/ __ \\_  __ \\"));
    println!("\x1B[31m{}\x1B[0m", format!(" |    |     / __ \\_\\___ \\ \\___ \\  \\     / /_/ | /_____/ |  Y Y  \\/ __ \\|   |  \\/ __ \\_/ /_/  >  ___/|  | \\/"));
    println!("\x1B[31m{}\x1B[0m", format!(" |____|    (____  /____  >____  >  \\/\\_/\\____ |         |__|_|  (____  /___|  (____  /\\___  / \\___  >__|           "));
    println!("\x1B[31m{}\x1B[0m", format!("                \\/     \\/     \\/             \\/               \\/     \\/     \\/     \\//_____/      \\/       "));
}

fn help() {
    println!("Utilisez les commandes suivantes :");
    println!("  - help   : Affiche ce message d'aide");
    println!("  - add    : Ajoute un nouvel utilisateur");
    println!("  - list   : Liste tous les utilisateurs");
    println!("  - secret : Affiche les mots de passe réels (accès réservé à root)");
    println!("  - remove : Supprime un utilisateur");
    println!("  - quit   : Quitte le programme");
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn my_loop() {
    start();

    let mut users: HashMap<usize, User> = HashMap::new();
    let mut user_counter: usize = 1;
    
    let current_dir = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
    let path_csv = format!("{}/data.csv", current_dir);
    let _ = csv_handle::create_csv(&path_csv);
    
    let mut key: String;
    
    loop {
        print!("Entrez votre clé de 16 caractères: ");
        io::stdout().flush().unwrap();
        key = get_input();
        if key.len() == 16 {
            break;
        }
        println!("Tu as entré {}", key.len());
    }

    println!("\nBienvenue dans le gestionnaire de mots de passe !");
    println!("\nLance la commande \"help\" pour savoir comment utiliser ce programme\n");    
    load_user(&path_csv, &mut users, &mut user_counter);
    loop {
        print!("{}> ", whoami::username().green());
        io::stdout().flush().unwrap();
        let _input = get_input();
        
        match _input.as_str() {
            "help" => help(),
            "add" => add_users(&mut users, &mut user_counter, key.to_string()),
            "list" => list_users(&users),
            "secret" => show_real_password(&users, key.to_string()),
            "remove" => remove_user(&mut users, key.to_string(), path_csv.to_string()),
            "quit" => break, 
            _ => println!("\x1B[31m{}\x1B[0m", format!("Commande inconnue"))

        }
    }
}

fn load_user(path: &str, users: &mut HashMap<usize, User>, counter: &mut usize) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut rdr = Reader::from_reader(reader);

    for result in rdr.records() {
        let record = result.unwrap();
        let number_id: usize = record[0].parse().unwrap();
        
        users.insert(number_id, User {
            app_name: record[1].to_string(),
            username: record[2].to_string(),
            password: record[3].to_string(),
        });
        *counter = number_id + 1;
    }
}

fn add_users(users: &mut HashMap<usize, User>, counter: &mut usize, key: String) {
    print!("Entrez le nom de l'app : ");
    io::stdout().flush().unwrap();
    let app_name: String = get_input();
    
    print!("Entrez le nom d'utilisateur : ");
    io::stdout().flush().unwrap();
    let username: String = get_input();

    print!("Entrez le mot de passe : ");
    io::stdout().flush().unwrap();
    let password: String = get_input();
    let password_crypt = encryption::encrypt_text(&key, &add_space(&password));
    
    
    users.insert(*counter, User {
        app_name: app_name.clone(),
        username: username.clone(),
        password: password_crypt.clone(),
    });

    let current_dir = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
    let path_csv = format!("{}/data.csv", current_dir);
    let _ = csv_handle::add_in_csv(&path_csv,
    counter, app_name.clone(), username.clone(), password_crypt.clone());

    
    *counter += 1;
    println!("{}", format!("Les données ont été ajouté avec succès").green());
}

fn list_users(users: &HashMap<usize, User>) {
    let mut table: Table = Table::new();
    table.add_row(row!["Numéro", "App", "Nom", "Password"]);

    let mut sorted_users: Vec<_> = users.iter().collect();
    sorted_users.sort_by(|a, b| a.0.cmp(b.0));

    for (index, user) in sorted_users {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()),
            Cell::new(&user.app_name),
            Cell::new(&user.username),
            Cell::new(&user.password),
        ]));
    }
    table.printstd();
}

fn show_real_password(users: &HashMap<usize, User>, key: String) {
    let mut table: Table = Table::new();
    table.add_row(row!["Numéro", "App", "Nom", "Password"]);

    let mut sorted_users: Vec<_> = users.iter().collect();
    sorted_users.sort_by(|a, b| a.0.cmp(b.0));

    for (index, user) in sorted_users {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()),
            Cell::new(&user.app_name),
            Cell::new(&user.username),
            Cell::new(&encryption::decrypt_text(&key, &user.password).trim()),
        ]));
    }
    unsafe {
        if libc::geteuid() == 0 {
            table.printstd();
        }
        else {
            println!("Tu n'y as accès qu'en tant que root");
        }
    }
}

fn load_data_in_csv(file_path: &str, users: &mut HashMap<usize, User>) -> Result<()> {
    let _ = remove_file(file_path);
    let mut sorted_users: Vec<_> = users.iter().collect();
    sorted_users.sort_by(|a, b| a.0.cmp(b.0));

    let _ = csv_handle::create_csv(file_path);

    for (index, user) in sorted_users {
        let _ = csv_handle::add_in_csv(file_path, index, user.app_name.to_string(), user.username.to_string(), user.password.to_string());
    }

    Ok(())
}

fn remove_user (users: &mut HashMap<usize, User>, key: String, path: String) {
    print!("Entrez le nom de l'app : ");
    io::stdout().flush().unwrap();
    let app_name: String = get_input();
    
    print!("Entrez le nom d'utilisateur : ");
    io::stdout().flush().unwrap();
    let username: String = get_input();

    print!("Entrez le mot de passe : ");
    io::stdout().flush().unwrap();
    let password: String = get_input();

    let user_id_to_remove = users.iter().find_map(|(user_id, user)| {
        if user.app_name == app_name && user.username == username && user.password == encryption::encrypt_text(&key, &add_space(&password)) {
            Some(*user_id)
        } else {
            None
        }
    });

    match user_id_to_remove {
        Some(user_id) => {
            users.remove(&user_id);
            let _ = load_data_in_csv(&path, users);
        }
        None => println!("{}", format!("Utilisateur non trouvé avec l'application, le nom d'utilisateur et le mot de passe spécifiés.").red()),
    }
    println!("{}", format!("Les données ont été supprimé avec succès").green());
}
