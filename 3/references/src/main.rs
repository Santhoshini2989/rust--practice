 //references and borrowing 


   //return multiple values using a tuple,

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


fn main(){
    leet s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
 
}
fn calculate_length(s:&String)->usize{
    s.len
}

//mutable references 

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

 /*

 referencing and dereferencs 
 data race -- Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.
-- we can avoid it by creating a scope

Dangling References - - a pointer that references a location in memory 
that may have been given to someone else—by freeing some memory
 while preserving a pointer to that memory
 

 fn main() {
    let reference_to_nothing = dangle();
}

 fn dangle() -> &String {
    let s = String::from("hello");

    &s == s
}

  */