 fn main(){
    let s = String::from("hello");
 }
 // string literals are not mutable o we have string type which mentioned above 

 fn main(){
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{s}");
 }
/* sallow copy,deep copy ,drop,move-copy,clone  */

 //cloning -- deep copy
 fn main(){
    let s = String::from("hello");
    let s2= s.clone();
    println!("s={s},s2={s2}");
 }

 //ownership and function
 fn main(){
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
 }
 fn takes_ownership(a:String){
  println!("{a}");
 }
 fn makes_copy(b:i32){
  println!("{b}");
 }

 //Return Values and Scope

  fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_gives_back(s2);
  }

  fn gives_ownership()->String{
   let some_string = String::from("yours");
   some_string
  }
  fn takes_gives_back(a_string:String)->String{
    a_string
  }

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