
// generics in function definition
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    //  +--  `&` changes `item` from `T` to `&T`. To use it, T must implement
    //  |    the copy trait. 
    //  v
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generics in enum definition
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// generics in struct definition
struct Point<T> {
    x: T,
    y: T,
}

// generics in method definition
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // only for Point<f32>
    fn y(&self) -> &f32 {
        &self.y
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    // Generic type parameters in a struct definition aren’t always the same as 
    // those you use in that struct’s method signatures. 
    // Some generic type parameters are declared with `impl`, while some others
    // are declared with the method definition. For example, <T, U> is needed 
    // here because Point2 struct definition requires two type parameters; 
    // <V, W> are declared after `fn`, so they are only relevant to the method.
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // There's no runtime cost for using generics. 
    // Rust does code generation at compile time.
}
