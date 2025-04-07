
pub fn main() {
    println!("Hello, world!.");

    //number
    let x:i32 = 5;
    println!("{}.", x);

    //bool
    let is_male = true;
    let is_above_18 = true;

    //conditional
    if is_male{
        println!("You are a male.");
    }
    else{
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male.");
    }

    //Strings
    let greeting = String::from("Hello world!.");
    println!("{}.", greeting);

    //Conditional with function is_even(x:i32) -> bool
    let y = 99;
    let is_even = is_even(y);
    if is_even {
        println!("{} is even.", y);
    }
    else{
        println!("{} is not even.",y);
    }

    // loops
    let str = String::from("Adepu Rahul.");
    println!("First Name is {}.", get_first_name(str));

    // funcitons
    let a:i32 = 69;
    let b:i32 = 69;
    println!("{} is the sum of {} and {}.", do_sum(a,b), a,b);

    //references and borrowing
    let mut s1 = String::from("Hello Rust");
    update_word(&mut s1);// it did not change the ownership, but the values in the place gets changed as it is mutable
    println!("{}", s1);

    //it cannot  be done because an mutable is being borrowed to an immutable
//    let mut y1 = String::from("Hello");
//    let y2 = &mut y1;
//    update_word(&mut y1);
//    println!("{}", y1);
//    println!("{}", y2);

    //Rules of references and borrowing
    // only one mutable reference should be there
    // we can have more than one immutable references
    // if there is a mutable reference then we cant have immutable references

    // here the third rule is breaking
//    let mut z1 = String::from("Hellow");
//    let z2 = &mut z1;
//    let z3 = &z1;
//    println!("{}", z2);
//    println!("{}", z3);

    //Structs in rust
//    let mut user1 = User{
//        active: true,
//        username: String::from("someName123"),
//        sign_in_count: 1,
//    };
//    user1.username.push_str("hello");
//    println!("{:?}", user1.username);
//    println!("{:?}", user1.active);
//    println!("{:?}", user1.sign_in_count);
//
//    change_name(user1.clone());
//    print!("{}", user1.active);

    let rect = Rect{
        width :30,
        height :40,
    };
    println!("The area of the rectangle is : {}", rect.area() );

    //pattern matching in rust
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);


    println!("Area of Circle: {}", calculate_area(circle));
    println!("Area of Square: {}", calculate_area(square));
    println!("Area of Rectangle: {}", calculate_area(rectangle));


}


pub fn is_even(x:i32) -> bool {
    return x%2==0;
}

pub fn get_first_name(str:String) -> String{
    let mut first_name = String::from("");
    for c in str.chars(){
        if c == ' '{
            break;
        }
        first_name.push(c);
    }
    return first_name;
}

pub fn do_sum(a:i32, b:i32) -> i32{
    return a+b;
}

pub fn update_word(some_string: &mut String) {
    some_string.push_str(" World");
}



//#[derive(Clone)]
//struct User {
//    active: bool,
//    username: String,
//    sign_in_count: u64,
//}
//pub fn change_name(user1: User){
//    print!("{:?}", user1.active);
//}


struct Rect{
    width: u32,
    height: u32,
}

impl Rect{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

//enum
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64{
    match shape{
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length*length,
        Shape::Rectangle(width, height) => width* height,
    }
}