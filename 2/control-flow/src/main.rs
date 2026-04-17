 //basic conditions - if   if else    else if

 
 fn main(){
    let number = 3;

    if number<5 {
        println!("condition is true");
    }
    else{
        println!("condition is false");

    }
 }

//Handling Multiple Conditions with else if

fn main(){
    let number = 6;

    if number%4 ==0{
        println!("numebr is divisible by 4");
    }
    else if number%3==0{
        println!("numebr is divisible by 3");

    }
    else if number% 2==0{
        println!("numebr is divisible by 2");

    } else{
        println!("numebr is not  divisible by 2,3,4");

    }
}

//using if in let statement 
fn main(){
    let  condition = true;
    let number = if condition{5}else{6};
    println!("the value of the nu,ber is;{number}");
}

//loops - - loop,while,for 
//loop - - it runs forever until we break it
fn main(){
    loop{
        println!("hello!");
        break;
    }
}

//Returning Values from Loops
 fn main(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter==10{
            break counter;
        }
    };
    println!("the result = {result}");
}

//Disambiguating with Loop Labels

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

//while -loop
fn main(){
    let mut number = 3;
    while number!=0{
        println!("{number}");
        number -=1;
    }
    println!("LIFTOFF!!");
}


//array related 
 fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}


//for - loop
fn main(){
    let a =[10,20,30,40,50];
    for element in a {
        println!("the Value is :{element}");
    }
}


// using for  instead of while 

fn main(){
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!!");
}