use std::io;

struct TodoItem {
    id: u64,
    name: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem {
            id,
            name: title.clone(),
            completed: false,
        };
        self.items.push(new_item);
        println!("Added item: {}", title);
    }

    fn remove_item(&mut self, id: u64) {
        let index = self.items.iter().position(|item| item.id == id);
        if let Some(index) = index {
            let removed_item = self.items.remove(index);
            println!("Removed item: {}", removed_item.name);
        } else {
            println!("Item with id {} not found.", id);
        }
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("No items found.");
        }
        println!("Your Todo List:");
        for item in &self.items {
            let status = if item.completed {"[X]"} else {"[ ]"};
            println!("{}: {} {}", status, item.id, item.name);
        }
    }

    fn complete_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.completed = true;
            println!("Marked item {} as completed.", id);
        } else {
            println!("Item with id {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    println!("Welcome to your Todo List!");
    loop {
        println!("Options:");
        println!("1. Add Item");
        println!("2. Remove Item");
        println!("3. List Items");
        println!("4. Complete Item");
        println!("5. Quit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match choice {
        1 => {
            println!("Enter the Title of the item to add:");
            let mut title = String::new();
            io::stdin().read_line(&mut title).expect("Failed to read line");
            todo_list.add_item(title.trim().to_string());
        }
        2 => {
            println!("Enter the ID of the item to remove:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id: u64 = id.trim().parse().expect("Please type a number!");
            todo_list.remove_item(id);
        }
        3 => {
            todo_list.list_items();
        }
        4 => {
            println!("Enter the ID of the item to complete:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id: u64 = match id.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            todo_list.complete_item(id);
        }
        5 => {
            println!("Goodbye!");
            break;
        }
        _ => println!("Invalid choice. Please try again."),
        }
    }
}
