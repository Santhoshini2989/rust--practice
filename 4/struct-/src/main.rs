 //blueprint -- defing a struct

 struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
 }

 //creating an instance
 fn main(){
    let user1 = User{
       active:true,
       username:String::from("pooja"),
       email:String::from("abc@gmail.com"),
       sigin_in_count:1,
    };
 }
 //just add mut to be ab;le to change 
 fn main(){
    let mut  user1 = User{
       active:true,
       username:String::from("pooja"),
       email:String::from("abc@gmail.com"),
       sigin_in_count:1,
    };
    user1.email = String::from("xyz@gmail.com");
 }
 //update syntax 

 //tuple structs 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//unit-like structs - - no fields  -- used when you wanna apply trait 

struct AlwaysEqual;
fn main(){
    let subject = AlwaysEqual;
}