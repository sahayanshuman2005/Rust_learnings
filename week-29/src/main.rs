// Annotations and decorators
// Annotations/Decorators serve to modify code behaviour in a declarative way without explicitly changing the core logic of a function or class. They often allow for additional functionality to be "applied" to a class, method, or variable. They’re all applied at runtime
// Decorators in python
// from fastapi import FastAPI

// app = FastAPI()

// @app.get("/items/{item_id}")
// async def read_item(item_id: int, q: str = None):
//     return {"item_id": item_id, "q": q}

// ​
// Annotations in typescript (nestjs)
// @Post()
// createItem(@Body() body: CreateItemDto) {
//   return this.itemsService.create(body);
// }
// ​
// Annotations in java
// import org.springframework.web.bind.annotation.GetMapping;
// import org.springframework.web.bind.annotation.RestController;

// @RestController
// public class ItemController {
//     @GetMapping("/items")
//     public List<String> getItems() {
//         return List.of("item1", "item2", "item3");
//     }
// }

// Macros
// macros are a powerful feature that allows for metaprogramming by enabling the generation of code at compile-time.

// declarative macro
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();  // Expands to: println!("Hello, world!");
}

// Procedural macro
// types of proc macro

// 1. custom derive macro
// #[derive(Serialize, Deserialize)]
// struct User {
// 	username: String,
// 	password: String,
// 	age: u32
// }


//2.  Attribute-like Macros:
// #[route("GET")]
// fn home() {
//     println!("Welcome to the home page!");
// }

// #[route("POST")]
// fn create_post() {
//     println!("Creating a new post!");
// }

// 3. function like macros
// pub fn my_vec(input: TokenStream) -> TokenStream {
//     let input = input.to_string();
//     let gen = format!("vec![{}]", input);
//     gen.parse().unwrap()
// } 

// use proc_macro::TokenStream;

// #[proc_macro]
// pub fn my_vec(input: TokenStream) -> TokenStream {
//     let input = input.to_string();
//     let gen = format!("vec![{}]", input);
//     gen.parse().unwrap()
// } 


// Macros applied to attributes
// cargo add serde serde_json
// // Update serde to use the derive feature
// serde = {version = "1.0.218", features = ["derive"]}

// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct User {
//     #[serde(rename = "user_name")]
//     username: String,
    
//     #[serde(rename = "pass_word")]
//     password: String,
    
//     #[serde(rename = "user_age")] 
//     age: u32,
// }

// fn main() {
//     let user = User {
//         username: String::from("Alice"),
//         password: String::from("password123"),
//         age: 30,
//     };

//     // Serializing to JSON
//     let json = serde_json::to_string(&user).unwrap();
//     println!("{}", json); 
//     // Prints: {"user_name":"Alice","pass_word":"password123","user_age":30}
// }

// Common Macro Functions in Rust
// println!
// println!("Hello, {}!", "world");
// ​
// vec!
// let v = vec![1, 2, 3, 4];
// ​
// format
// let formatted = format!("{} {}", "Hello", "world");
// ​
// assert
// assert!(5 > 3);
// ​
// panic
// panic!("This is a panic!");

// Implementing the Display trait manually
// impl std::fmt::Display for Rect {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.width, self.height)
//     }
// }

//Example of Copy:
fn main() {
    let a = 5;  // `i32` implements `Copy`
    let b = a;  // `a` is copied into `b`
    println!("a: {}, b: {}", a, b);  // This works, because `a` is not moved, it's copied (no ownership transferred)
}
 
//Example of Clone:

#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person2 = person1.clone();  // Explicitly cloning `person1`

    println!("person1: {}, {}", person1.name, person1.age);
    println!("person2: {}, {}", person2.name, person2.age);
}
