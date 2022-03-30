
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// 튜플 구조체
// struct Color(i32, i32, i32); 
// struct Point(i32, i32, i32);
//

// fn build_user(email: String, username: String) -> User {
//     User { 
//         email, 
//         username, 
//         sign_in_count: 1, 
//         active: true
//     }
// }

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return self.length > rect.length && self.width > rect.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main()
{
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    let rect4 = Rectangle::square(50);

    println!(
        "The area of the rect4 is {:#?} square pixels.", 
        rect4);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
