fn main(){
    // numbers
    let a:i32 = 1;
    println!("{}",a);

    //booleans && conditionals
    let is_male= true;
    let is_above_18=false;
    if is_male{
        println!("you are male");
    }else{
        println!("you are not male");
    }
    if is_male && is_above_18{
        println!("you are legal male");
    }
    // strings
    let name= String::from("ansh");
    println!("{}",name);

    //vectors
    let v= vec![1,2,3];
    println!("{:?}",v);

    // arrays
    let arr:[i32; 5] =  [1,2,3,4,5];
    println!("{}",arr.len());

    // loops
    for i in 0..10{
        println!("{}",i);
    }

    // mutable and immutable var.
    // by default, all variables in rust are immutable
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");
}