// needed to print the struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// A unit struct
struct Unit;

struct Point {
    x: f32,
    y: f32
}

// A tuple struct
struct Pair(i32, f32);

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { bottom_right: Point { x: bottom_right_x, y: bottom_right_y}, top_left: Point { x: top_left_x, y: top_left_y}} = rectangle;

    return (top_left_x - bottom_right_x) * (top_left_y - bottom_right_y);
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };
    let another_point = Point { x: 5.2, y: 0.2 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..another_point };

    // `bottom_right.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area {:?}", rect_area(rectangle));
}
