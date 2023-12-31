// This is the entry-point of the rust application
fn main() {
    println!("Hello, world!");

    let mut i = 0;
    // it's a infinite loop, where we need a breaking condition inside it
    loop {
        if i >= 10 {
            break;
        }

        println!("This is Shashi");
        println!("{:?}", i);
        i = i + 1;
    }

    let mut j = 0;

    // while loop is just like any other while loop in other langugaes
    while j <= 3 {
        println!("{:?}", j);
        j += 1;
    }

    second_method();
    print_first_name();
    print_last_name();
    println!("{:?}", add(3, 4));
    match_switch();
    get_direction(Direction::Left);
    create_person();
    use_fn();
    tuples();
    expression();

    let string_value: String = String::from("This is actual");
    test_ownership(&string_value);
    test_ownership(&string_value);

    let hot = Temperature { degrees_f: 99.9 };
    show_temp(hot);

}

fn second_method() {
    let my_favorite_color = "blue";
    println!("{:?}", my_favorite_color);
}

fn print_first_name() {
    println!("Shashi");
}

fn print_last_name() {
    println!("Bhagat");
}

// to run we use the below command
// cargo run --bin main
// -q flag can be used for to remove boiler plate logging

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn match_switch() {
    let some_bool: bool = true;
    match some_bool {
        true => println!("This is true"),
        false => println!("This is false"),
    }

    let some_int: i32 = 6;
    match some_int {
        1 => println!("The selected number is 1"),
        2 => println!("The selected number is 2"),
        3 => println!("The selected number is 3"),
        4 => println!("The selected number is 4"),
        5 => println!("The selected number is 5"),
        _ => println!("Invalid entry"),
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_direction(go: Direction) {
    match go {
        Direction::Up => println!("up"),
        Direction::Right => println!("right"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
    }
}

// Structure (struct)
//It is used to create new data types

// example
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn struct_exmp() {
    let my_box = ShippingBox {
        depth: 3,
        width: 5,
        height: 7,
    };

    let tall = my_box.height;
    println!("{tall_}", tall_ = tall)
}

struct Person {
    first_name: String,
    last_name: String,
    email_id: String,
    age: i32,
    height: i32,
}

fn create_person() {
    let first_person: Person = Person {
        first_name: String::from("Shashi"),
        last_name: String::from("Bhagat"),
        email_id: String::from("skujur871@gmail.com"),
        age: 29,
        height: 174,
    };

    println!(
        "Person name: {f_name} {l_name}",
        f_name = first_person.first_name,
        l_name = first_person.last_name
    );
    println!(
        "Height is : {height},\nEmail id is {email},\nAge is {age}",
        height = first_person.height,
        email = first_person.email_id,
        age = first_person.age
    );

    struct_exmp();
}

// 11/06/2023
fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn use_fn() {
    let a: i32 = 10;
    let b: i32 = 5;

    let ans: i32 = sub(a, b);
    println!("The sub of 10 and 5 is :{answer}", answer = ans)
}

fn tuples() {
    let person_details: (&str, &str, i32, &str) = ("Shashi", "skujur871@gmail.com", 29, "Male");
    let (name, email, age, gender) = person_details;

    println!("{:?}, {:?}, {:?}, {:?}", name, email, age, gender);
}

// Demo Exp

enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn expression() {
    let access_level = Access::Guest;
    let can_access_level = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("{:?}", can_access_level)
}

// & key is used as a suffix to tell the program that we are borrowing the variable and please don't delete it

fn test_ownership(a: &String) {
    println!("{:?}", a);
}

// impl keyword

struct Temperature {
    degrees_f: f64,
}

fn show_temp(temp: Temperature) {
    println!("{:?} degrees F", temp.degrees_f);
}

