struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64, // Comma even after the last element
}

struct Colour(i32, i32, i32); // tuple struct
struct Coord(i32, i32, i32); 

struct ExUnitLikeStruct; // Example unit-like struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Section 1");
    instantiating();
    tuple_and_unit_like();
    println!("\nSection 2");
    area_of_rect();
}

fn area_of_rect() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30,50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Print Debug - Using :? rect is {:?}",rect2);
    println!("Print Debug - Using :#? rect is {:#?}",rect2);
    println!(
        "The area of the rectangle is {} or {} or {} or {} square pixels",
        area(width1, height1),
        area_tuple(rect1),
        area_struct(&rect2),
        rect2.area()
        );
    if rect2.width() {
        println!("Width of the rectangle is {}", rect2.width);
    }
    print!("Rectangle 2 {:?} can",rect2);
    if !rect2.can_hold(&rect3) {
        print!("not")
    }
    println!(" hold Rectangle 3 {:?}",rect3);
    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    height * width
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn tuple_and_unit_like() {
    // Example, setting the colour of a point. And mentioning a point in 3d space
    let _black = Colour(0,0,0); // Essentially set a triplet in a variable. Imagine struct, but no names
    let _origin_3d = Coord(0,0,0);
    let _testus = ExUnitLikeStruct;
}

fn instantiating() {
    println!("STRUCTURES!!!");
    let mut user1 = new_user("email@example.com".to_string(), "user_name".to_string());
    println!("Email - {0}",user1.email);
    user1.email = String::from("test-email@example.com");
    let user2 = User {
        email: String::from("test@email.com"),
        ..user1
    };
    println!("Edited email - {0}", user1.email); // Only value not moved in user2 assignment
    //print_struct(&user1); // fails because values were partially moved in user2 assignment
    println!("User 2 - ");
    print_struct(&user2);
    println!("\nUser 2 -\n\tname - {0}\n\temail - {1}\n\tactive - {2}\n\tsign in count - {3}", user2.username, user2.email, user2.active, user2.sign_in_count);
}

fn new_user(email: String, username: String) -> User {
    User {
        email, // email: email,
        username, // username: username,
        active: true,
        sign_in_count: 0,
    }
}

fn print_struct(printme: &User) {
    println!("\tUsername      - {0}",printme.username);
    println!("\tEmail         - {0}",printme.email);
    println!("\tActive        - {0}",printme.active);
    println!("\tSign in count - {0}",printme.sign_in_count);
}
