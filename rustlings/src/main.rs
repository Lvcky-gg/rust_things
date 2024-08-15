fn main() {
    let var = 1; // created on stack
    let mut s = "hello".to_string(); //created on heap
    s.push_str(", world");
// taking ownership example
    let x = vec!["john".to_string()];
    let y = x;
    let z = y;
    println!("{:?}",z);
//clone example, creates a deep copy. can be expensive operation
    let a = vec!["John".to_string()];
    let b = a.clone();
    let c = b.clone();
    println!("{:?}",c);
//copy can be implemented on types stored on stack

//taking ownership, a move
    let s = String::from("takes"); //create a variable with a string
    takes_ownership(s); // give ownership to function

    let val = 1;
    make_copy(val); //some types implement a copy, but dont give ownership

    let str1: String = give_ownership();
    println!("{}",str1);

    let str3: String = take_and_give(str1);
    println!("{}", str3);

}

fn takes_ownership(s: String){
    let string = s;
    println!("{}", string);
}

fn make_copy(one: i32){
    let val1 = one;
    println!("{}", val1);
}

fn give_ownership()-> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String{
    str2
}
//var is dropped, s is dropped

