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
//copy is for values of known size

//taking ownership, a move
    let s = String::from("takes"); //create a variable with a string
    takes_ownership(s); // give ownership to function
                        
// don't always take ownership, try to use references
// & immutable read only pointer
// rust does management through types over pointer math
// passing a pointer is way cheaper than taking ownership
    let val = 1;
    make_copy(val); //some types implement a copy, but dont give ownership
                    //stack allocated generally can be copied

    let str1: String = give_ownership();
    println!("{}",str1);

    let str3: String = take_and_give(str1);
    println!("{}", str3);

    loop{
        str2 = str1;
    }
    
    //this cannot be given out elsewhere
    // cannot return reference to a stack value from a function
    let mut str4: String = give_ownership();

 //   if(true){
    //    let str5 = str3;
    //} else {
      //  let str5 = str3;
    //}

    //println("{}", str3);
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
//variable is dropped by scope
//mutable references act differently, mutable reference is exclusive. only one value can hold it
