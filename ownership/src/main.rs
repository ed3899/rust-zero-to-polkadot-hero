use std::io;

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n1) Add todo\n2) List todos\n3) Quit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut task = String::new();
                println!("Enter task:");
                io::stdin().read_line(&mut task).unwrap();
                todos.push(task.trim().to_string()); // ownership of String moves into Vec
            }
            "2" => {
                println!("--- Todos ---");
                for (i, todo) in todos.iter().enumerate() {
                    println!("{}. {}", i + 1, todo); // borrowing immutably
                }
            }
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}