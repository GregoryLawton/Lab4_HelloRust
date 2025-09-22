/* Question 2 -Rectangle
struct Rectangle {
    width: f64, //width of the rectangle
    height: f64 //height of the rectangle
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle { //constructor method to create a new Rectangle instance
        Rectangle {width, height} //returns a new Rectangle instance with the given width and height
    }
    fn area(&self) -> f64 { //method to calculate the area of the rectangle
        self.width * self.height //returns the area by multiplying width and height
    }
    fn perimeter(&self) -> f64 { //method to calculate the perimeter of the rectangle
        self.width * 2.0 + self.height * 2.0 //returns the perimeter by adding twice the width and twice the height
    }
    fn is_square(&self) -> bool { //method to check if the rectangle is a square
        self.width == self.height //returns true if width and height are equal, otherwise false
    }
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0); //creates a new Rectangle instance with width 10.0 and height 5.0
    println!("Area: {}", rect.area()); //calls the area method to calculate and print the area of the rectangle
    println!("Perimeter: {}", rect.perimeter()); //calls the perimeter method to calculate and print the perimeter of the rectangle
    println!("Is Square? {}", rect.is_square()); //calls the is_square method to check and print if the rectangle is a square
    assert!(Rectangle::new(5.0, 5.0).is_square()); //asserts that a rectangle with equal width and height is a square
    assert!(!Rectangle::new(5.0, 6.0).is_square()); //asserts that a rectangle with unequal width and height is not a square
}
*/    

struct Circle {
    radius: f64, //radius of the circle //assume radius is > 0
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle {radius}
    }
    fn area(&self) -> f64 {
        3.14 * (self.radius * self.radius)
    }
    fn circumference(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}

fn main() {
    let circle = Circle::new();
    println!("Area: {}", circle.area()); //calls the area method to calculate and print the area of the circle
    println!("circumference: {}", circle.circumference()); //calls the circumference method to calculate and print the circumference of the circle
    assert_eq!(Circle::new(2.0).area(), 12.56); //asserts that a circle with radius 2.0 has area 12.56
    assert_eq!(Circle::new(2.0).circumference(), 12.56); //asserts that a circle with radius 2.0 has a circumference 12.56
}