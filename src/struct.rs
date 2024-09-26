struct User{
    first_name:String,
    last_name:String,
    age:i32
}
fn main(){
    let user = User{
        first_name:String::from("chirag"),
        last_name:String::from("kotian"),
        age:23
    };
    println!("first name:{}",user.first_name);
    println!("last name:{}",user.last_name);
    println!("age:{}",user.age);
}