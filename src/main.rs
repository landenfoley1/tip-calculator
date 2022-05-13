use std::io;
fn main(){
    let res:u64;
    let num1:u64;
    let num2:u64;
    let num3:u64;
    let mut input = String::new();
    let mut a_input = String::new();
    let mut b_input = String::new();
    
    println!("tip calculator");
    println!("how much is your bill? ");
    io::stdin().read_line(&mut input).expect("Not a valid int");
    println!("how many people are in your party? ");
    io::stdin().read_line(&mut a_input).expect("Not a valid int");
    println!("what percent would you like to tip? ");
    io::stdin().read_line(&mut b_input).expect("Not a valid int");
    num1 = input.trim().parse().expect("Not a valid number");
    num2 = a_input.trim().parse().expect("Not a valid number");
    num3 = b_input.trim().parse().expect("Not a valid number");  
    res = num1 * num3 / 100 / num2;
    print!("you need to tip {} dollars per person.",res);
}


