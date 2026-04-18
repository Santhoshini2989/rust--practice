 /*methopds are same like the functions 
 methods are defined within the context of a struct (or an enum or a trait object

the first parameter is always - self */

// example 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

/*Note that we can choose to give a method the same name as one of the struct’s fields.
 For example, we can define a method on Rectangle that is also named width: */

 impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
/*wehn we use the same method name and field name we offen want teh foirld value some ex 
are getters -- it makes the field valur private and the method value public so makes only read only not modify */



/*This time we want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle); otherwise,
 it should return false. That is, once we’ve defined the can_hold method,  */
  fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
