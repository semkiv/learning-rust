// Only "object-safe" traits can be used as trait objects.
// Basically the requirements boil down to the following two:
//     * the return type isn’t `Self`,
//     * there are no generic type parameters.
// This is somewhat analogous to C++ where cannot have virtual template functions (the second requirement) and if you return `*this` by value in a parent class you'll get the object of the child classes returned sliced (Rust simply disallows this by the first requirement)
pub trait Draw {
    fn draw(&self);
}

pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>, // `Box<dyn Draw>` is a so-called trait object - a stand-in for any object that implements `Draw` trait; since the actual objects that will be stored in the vector are determined at runtime, we can only store pointers like `Box<dyn Draw>`, `&dyn Draw` etc.
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            components: Vec::new(),
        }
    }

    pub fn push(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    dimensions: Dimensions,
    label: String,
}

impl Button {
    pub fn new(dimensions: Dimensions, label: String) -> Button {
        Button { dimensions, label }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button labeled {} with dimensions {}x{}px",
            self.label, self.dimensions.width, self.dimensions.height
        );
    }
}

pub struct SelectBox {
    dimensions: Dimensions,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(dimensions: Dimensions, options: Vec<String>) -> SelectBox {
        SelectBox {
            dimensions,
            options,
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a SelectBox with options {} and dimensions {}x{}px",
            self.options.join(", "),
            self.dimensions.width,
            self.dimensions.height
        );
    }
}
