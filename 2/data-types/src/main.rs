 fn main({
    let mut x:u32 = 55;
    println!("{x}");
 }

 fn main(){
    let mut y:f32 = 3.55;
    println!("{y}");
 }

 fn main(){
    let is_age :bool = true;
    println!("{is_age}");
 
 }

 // compounnd data types 

//tuple

 fn main(){
   let x:(i32,f64,u8)=(500,6.4,1);
   let a = x.0;
   let b = x.1;
   let c = x.2;
   println!("{a},{b},{c}");
 }

 //tuple destructuring 
   
 fn main(){
    let tup = (500,6.4,1);
    let (x,y,z)= tup;
    println!("{y}");
 }

  
 //array
 fn main(){
    let a =[1,2,3,4,5,6];
 }

  fn main(){
    let b :[i32;5]=[1,2,3,4,5];
  }