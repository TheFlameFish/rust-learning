#[derive(Debug)] // Allows the use of :? as shown on line 10
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // implementation
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width >= rect2.width && self.height >= rect2.height
    }

    fn square(length: u32) -> Rectangle {
        Rectangle {width: length, height: length}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};

    println!("The rectangle is {:#?}.", rect1); // :#? tells it to use debug formatting. :? is simple, :#? has fancy newlines and indenting.

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle {width: 25, height: 30};
    let rect3 = Rectangle {width: 100, height: 100};

    println!("Can rect1 fit in rect2? {}", rect2.can_hold(&rect1));
    println!("Can rect2 fit in rect1? {}", rect1.can_hold(&rect2));
    println!("Can rect1 fit in rect3? {}", rect3.can_hold(&rect1));

    let square = Rectangle::square(50);
    println!("\nSquare with a side length of 50 pixels: {:#?}", square);
}