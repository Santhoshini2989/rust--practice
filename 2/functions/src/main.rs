// normal  function

fn main(){
    println!("Hello World");
    another_function();
 }

 fn another_function(){
    println!("Another function");
 }

 //function with passing value 

 fn main(){
   println!("Hello world");
   another_function(5);
 }

 fn another_function(x:i32){
   5
 }
 //multiple parameters 
  
  fn main(){
   println!("Hello World");
   another_function(5,'h');
  }
  fn another_function(x:i32,y:char){
   println!("{x},{y}");
  }
//with returntype 

fn five()->i32{
55
}

fn main(){
   println!("hello");
   let x = five();
   println!("{x}");
}
// with parameters and return value 
 fn five(x:i32)->i32{
 x+5
}

fn main(){
   println!("hello");
   let x = five(10);
   println!("{x}");
}