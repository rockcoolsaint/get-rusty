struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // fn square_type(size: u32) -> Rectangle {
    //     Rectangle {
    //         width: size,
    //         height: size
    //     }
    // }

    fn square(size: u32) -> u32 {
        size * size
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("rockcoolsaint@gmail.com"),
        username: String::from("rockcoolsaint"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("rockcoolsaint2");
    
    let user2 = build_user(
        String::from("rockcoolsaint3@gmail.com"),
        String::from("rockcoolsaint3")
    );

    let user3 = User {
        email: String::from("rockcoolsaint4@gmail.com"),
        username: String::from("rockcoolsaint4"),
        ..user2
    };

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(25);

    println!("rect: {:?}", rect);

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels,",
        rect.area()
    );

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect1: {}", rect.can_hold(&rect2));

    println!(
        "The area of the square is {} square pixels,",
        rect3
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
