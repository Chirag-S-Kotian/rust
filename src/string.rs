fn main(){
    let name=String::from("chiragskotian");
    let len = get_str_len(name);
    println!("the length if the string is:{}",len);
}

// Function to calculate length of a string
fn get_str_len(str: String) -> usize{
    str.chars().count()
}