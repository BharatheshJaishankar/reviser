use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind, Read, Write},
};
struct Mcq {
    question: String,
    a: String,
    b: String,
    c: String,
    d: String,
    correct: String,
}

fn main() {
    let mut database: Vec<Mcq> = Vec::new();
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(r"C:\Users\Bharathesh\Desktop\release\mcq.txt")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::PermissionDenied {
                panic!("Permisson Denied");
            }
            if error.kind() == ErrorKind::Other {
                panic!("Failed due to some reason");
            } else {
                panic!("Failed");
            }
        });
    println!(
        "Select The Database:
    1 => Current Database
    2 => Load Database\n"
    );
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let input: i32 = answer.trim_end().parse().expect("Invalid Input");
    clearscreen::clear().unwrap();
    match input {
        1 => load_mcq(&mut file, &mut database),
        2 => load_database(&mut database),
        _ => panic!("Invalid Response/nShuting Down..."),
    };
    render_mcq(&mut database);
}

fn render_mcq(database: &mut [Mcq]) {
    let mut score: usize = 0;
    let mut result = String::new();

    clearscreen::clear().unwrap();
    for (questions, databasea) in database.iter().enumerate() {
        let space = "            ";
        if questions == 0 {
            result = format!("0/{}", database.len());
        }
        println!(
            "{}{}Revise App By Bharathesh Jaishankar{}Alpha Build\n",
            result, space, space
        );
        println!(
            "{}. {}\n  a) {}\n  b) {}\n  c) {}\n  d) {}\n",
            questions + 1,
            databasea.question,
            databasea.a,
            databasea.b,
            databasea.c,
            databasea.d
        );
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim_end() == databasea.correct {
            score += 1;
            result = format!("{}/{}", score, database.len());
        }
        clearscreen::clear().unwrap();
        if questions + 1 == database.len() {
            println!("Score: {}/{}", score, questions + 1);
            result = format!("Score: {}/{}", score, questions + 1);
        }
    }
}

fn load_mcq(file: &mut File, database: &mut Vec<Mcq>) {
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid File");
    if data.len() < 3 {
        let mut response = String::new();
        println!("New Database Created\nLoading the Database...\n\nEnter the Path to the Database");
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to Read");
        response = response.trim_end().to_string();
        let mut file_load = OpenOptions::new()
            .write(true)
            .read(true)
            .open(response)
            .unwrap_or_else(|error| {
                if error.kind() == ErrorKind::PermissionDenied {
                    panic!("Permisson Denied");
                }
                if error.kind() == ErrorKind::NotFound {
                    panic!("File Not Found");
                }
                if error.kind() == ErrorKind::Other {
                    panic!("Failed due to some reason");
                } else {
                    panic!("Failed");
                }
            });
        file_load.read_to_string(&mut data).expect("Invalid File");
        file.set_len(0).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        println!("Writing...");
    }
    let sad: String = String::from("\r\n\r\n");
    let split: Vec<&str> = data.split(&sad).collect();

    for (_iterator, splitted) in split.iter().enumerate() {
        let question: Vec<&str> = splitted.split("\r\n").collect();
        database.push({
            Mcq {
                question: question[0][3..].trim_start().to_string(),
                a: question[1][3..].to_string(),
                b: question[2][3..].to_string(),
                c: question[3][3..].to_string(),
                d: question[4][3..].to_string(),
                correct: question[5][8..].to_string(),
            }
        });
    }
}

fn load_database(database: &mut Vec<Mcq>) {
    let mut data = String::new();
    let mut response = String::new();
    println!("Enter the Path to the Database");
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to Read");
    response = response.trim_end().to_string();
    let mut file_load = OpenOptions::new()
        .write(true)
        .read(true)
        .open(response)
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::PermissionDenied {
                panic!("Permisson Denied");
            }
            if error.kind() == ErrorKind::NotFound {
                panic!("File Not Found");
            }
            if error.kind() == ErrorKind::Other {
                panic!("Failed due to some reason");
            } else {
                panic!("Failed");
            }
        });
    file_load.read_to_string(&mut data).expect("Invalid File");

    let sad: String = String::from("\r\n\r\n");
    let split: Vec<&str> = data.split(&sad).collect();

    for (_iterator, splitted) in split.iter().enumerate() {
        let question: Vec<&str> = splitted.split("\r\n").collect();
        database.push({
            Mcq {
                question: question[0][3..].to_string(),
                a: question[1][3..].to_string(),
                b: question[2][3..].to_string(),
                c: question[3][3..].to_string(),
                d: question[4][3..].to_string(),
                correct: question[5][8..].to_string(),
            }
        });
    }
}
