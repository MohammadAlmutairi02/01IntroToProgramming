use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // Create file
    let mut file = File::create(filename).expect("Unable to create file");

    // Write each book as a line
    for book in books {
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        file.write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut books: Vec<Book> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 3 {
            let book = Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse::<u16>().unwrap_or(0),
            };

            books.push(book);
        }
    }

    books
}
fn main() {
    let books = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            year: 1949,
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            year: 1960,
        },
    ];

    // Save books
    save_books(&books, "books.txt");
    println!("Books saved to file.");

    // Load books
    let loaded_books = load_books("books.txt");

    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}