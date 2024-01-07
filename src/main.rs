#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() >= rect.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: dbg!(5),
        height: 10
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // Debug macro
    dbg!(&rect1);

    println!(
        "The area of the rectangle (5x10) is {} square pixels.",
        rect1.area()
    );
    println!(
        "Is the width valid? {}",
        rect1.width()
    );

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

    let square1: Rectangle = Rectangle::square(5);
    println!("Area of a 5x5 square: {}", square1.area());
}