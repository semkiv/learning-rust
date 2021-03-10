use core::fmt::Display;

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

    println!(
        "{:?} {:?} (distance from origin: {}) {:?} {:?}",
        integer_point,
        float_point,
        float_point.distance_from_origin(),
        mixed_point,
        integer_point.mixup(&float_point),
    );

    let pair = Pair::new(10, 20);
    pair.cmp_display();
}

// note that a slice is used as a the parameter; this makes the function more generic than specifying container type directly
// comparisons (`<`) are not available for every possible type `T` so it won't compile unless we restrict the `T` to the types that implement `PartialOrd` (and thus `<` binary operation)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// `impl` block should contain generic type parameters too when it's the implementation of generic behaviour
// by declaring `T` and U` as a generic types after `impl`, Rust can identify that the types in the angle brackets in `Point` are generic types rather than concrete types
impl<T: Clone, U: Clone> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    // there can be generic functions inside the specialization too
    fn mixup<V: Clone, W: Clone>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

// however `impl` can be used without those for concrete type specializations, e.g. `Point<f64, f64>`
impl Point<f64, f64> {
    // only specialization of `Point<f64, f64>` will have `distance_from_origin` method
    fn distance_from_origin(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
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
