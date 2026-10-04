// // external packages / crates
// // cargo add module_name

// //some local packages
// // chrono,dotenv,uuid, tui, thiserror, sqlx

// // Chrono lets you do data and time in rust

// // You can get the current date and time in the UTC time zone (Utc::now()) or in the local time zone (Local::now()).

// // use chrono::prelude::*;

// // let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`

// use chrono::prelude::*;

// fn main() {
//     let local: DateTime<Local> = Local::now();
//     println!("{}", local);
// }// e.g. `2014-11-28T21:45:59.324310806+09:00`

// // Generics and trait bounds
// use std::fmt::Display;

// fn print_value<T: Display>(value: T) {
//     println!("{}", value);
// }

// fn add<T>(a: T, b: T) -> T
// where
//     T: std::ops::Add<Output = T>,
// {
//     a + b
// }

// fn main() {
//     print_value(100);
//     print_value("Hello Rust");

//     let result = add(10, 20);
//     println!("{}", result);
// }

// // Generics over structs
// struct Rect<T> {
//     width: T,
//     height: T,
// }

// impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
//     pub fn area(&self) -> T {
//         return self.height * self.width
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 10,
//         height: 20
//     };

//     println!("{}", r.area());
// }

// // generics over enums
// // = result enum, option enum etc

// // Traits
// trait Shape {
//     fn area(&self) -> f32;
// }

// Structs cam implement these traits

// struct Rect {
//     width: f32,
//     height: f32
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height
// 	  }
// }


