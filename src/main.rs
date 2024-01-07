fn main() {
    let scale = 2;
    let rect = Rectangle {
        height: dbg!(10 * scale),
        width: 20
    };
    // dbg!(&rect);
    //
    // println!("rect1 is {:#?}", rect);

    let ar = rect.area();
    println!("The area of the rectangle is  {ar}");
    let rect2 = Rectangle::square(10);
    let hold = rect.can_hold(&rect2);
    println!("can hold - {hold}");
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}