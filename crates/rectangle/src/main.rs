#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn print_dimensions(&self) {
        println!(
            "The area of the the rectangle is {} square pixels \n {:#?}",
            self.area(), 
            self
        );
    }
}

fn main() {
    let rect1 = Rectangle::square(100);
    rect1.print_dimensions();


    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    rect2.print_dimensions();

    println!("Rectangle one can hold rectangle two? {}", rect1.can_hold(&rect2));
}
