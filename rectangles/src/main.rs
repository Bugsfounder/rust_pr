fn area(dimention: (u32, u32)) -> u32 {
    dimention.0 * dimention.1
}

fn areaStruct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
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

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, anotherRect: &Rectangle) -> bool {
        self.width > anotherRect.width && self.height > anotherRect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: 32,
            height: 43,
        }
    }
}
fn main() {
    let rect1 = (21, 43);
    println!("the area of the rectangle: {}", area(rect1));

    let rect1 = Rectangle {
        width: 21,
        height: 43,
    };
    println!("Area of rectangle is: {}", areaStruct(&rect1));
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);

    // using method area associated with Rectangle
    println!("{}", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}", rect1.can_hold(&rect2));
}
