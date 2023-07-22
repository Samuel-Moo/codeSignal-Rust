#![allow(warnings)]
//add
fn add(param1: i32, param2: i32) -> i32 {
    param1 + param2
}

//centuryFromYear
fn century_from_year(year: i32) -> i32 {
    let mut century = 1;
    
    while century*100 < year{
        century += 1;
    }
    
    century
    
}

fn main() {
    //Execute Stuff
    let function_to_test = century_from_year(1898); 
    
    println!("Result: {}", function_to_test);
}


