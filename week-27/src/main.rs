// //borrowing
// fn main(){
//     let str =  String::from("Anshuman");
//     let len= get_length(&str);
//     println!("{} {}",str,len);
// }
// fn get_length(str: &String)->usize{
//     let len = str.len();
//     return len
// }

// //mutable and immutable references
// fn main(){
//     let s1= String::from("Ansh");
//     let s2= &s1;
//     let s3= &s1;
//     let s4= &s1;
//     println!("{} {} {} {}",s1,s2,s3,s4);
// }
// // you can have multiple immutable references

// // Structs
// struct User{
//     active: bool,
//     username:String,
//     email:String,
//     sign_in_count:u64
// }
// fn main(){
//     let user1=User{
//         active:true,
//         username: String::from("ansh"),
//         email: String::from("ansh421@gmail.com"),
//         sign_in_count:1,
//     };
//     print!("User 1 username: {:?}", user1.username);
// }

// // enums,pattern matching(using match => if ye then ye), 
// enum Direction{
//     North,
//     South,
//     East,
//     West
// }
// fn main(){
//     let direction=Direction::South;
//     steer(direction);
// }
// fn steer(dir:Direction){
//     match dir{
//         Direction::North => print!("North"),
//         Direction::South => print!("South"),
//         _ => println!("horizontal direction"), 
//     }
// }


// use std::f32::consts::PI;
// // enums with values
// enum Shape{
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32)
// }

// fn main(){
//     let shape =  Shape::Square(10.0);
//     let shape_circle = Shape::Circle(10.0);
//     let shape_rect = Shape::Rectangle(10.0, 10.0);
//     print!("{}",calculate_area(shape)); 
// }
// fn calculate_area(s:Shape)-> f32{
//     match s{
//         Shape::Circle(r)=> PI*r*r,
//         Shape::Rectangle(a,b )=>a*b,
//         Shape::Square(x)=>x*x,
//     }   
// }


// // error handling ...result enum
// use std::fs;

// fn main() {
//     let greeting_file_result = fs::read_to_string("hello.txt");

//     match greeting_file_result {
//         Ok(file_content) => {
//             println!("File read successfully: {:?}", file_content);
//         },
//         Err(error) => {
//             println!("Failed to read file: {:?}", error);
//         }
//     }
// }

// // option enum
// fn find_first_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("ansh");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is found at index: {}", index),
//         None => println!("The letter 'a' is not found in the string."),
//     }
// }