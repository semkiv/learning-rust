use core::fmt::Display;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Clone, U: Clone> Point<T, U> { // `impl` block should contain generic type parameters too when it's the implementation of generic behaviour; however it can be used without those for concrete type specializations, e.g. `Point<f32, f32>`
    fn _x(&self) -> &T {
        &self.x
    }

    fn _y(&self) -> &U {
        &self.y
    }

    fn mixup<V: Clone, W: Clone>(&self, other: &Point<V, W>) -> Point<T, W> { // there can be generic functions inside the specialization too
        Point {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

// Pair with all types will have `new` function.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// `cmp_display` function will be available only for those types that implement both `Display` and `PartialOrd`
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T { // note that a slice is used as a the parameter; this makes the function more generic than specifying container type directly
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed_point = Point { x: 0.1, y: 10 };

    println!("{:?} {:?} {:?} {:?}", integer_point, float_point, mixed_point, integer_point.mixup(&float_point));

    let pair = Pair::new(10, 20);
    pair.cmp_display();
}
