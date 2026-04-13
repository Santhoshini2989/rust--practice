 //shadowing is creating different var with same var name and it has power to change the data type too 
 // mostly used in trasforming the data 
 fn main(){
  let x = 5;
  println!("{x}"); 
  let x = x+5;      //new variable
  println!("{x}");
 }