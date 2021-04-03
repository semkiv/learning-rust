// No matter what, Safe Rust can't cause Undefined Behavior.
// This is referred to as soundness: a well-typed program actually has the desired properties.
// To ensure soundness, Safe Rust is restricted enough that it can be automatically checked.
// Sometimes, however, it is necessary to write code that is correct for reasons which are too clever for the compiler to understand.
// In those cases, you need to use Unsafe Rust.

// Here are the abilities Unsafe Rust has in addition to Safe Rust (so-called "unsafe superpowers"):
// * Dereference raw pointers
// * Implement unsafe traits
// * Call unsafe functions
// * Mutate `static`s (including `external` ones)
// * Access fields of `union`s

static mut COUNTER: u32 = 0; // the names of static variables are in SCREAMING_SNAKE_CASE by convention, just like of `const`s

fn main() {
    Foo::raw_pointers();
    Foo::unsafe_functions();
    Foo::statics();
    Foo::union_fields();
}

// the `unsafe` keyword can be used to declare the existence of contracts the compiler can't check like with `unsafe trait`
unsafe trait UnsafeSuperpowers {
    fn raw_pointers();
    fn unsafe_functions();
    fn statics();
    fn union_fields();
}

struct Foo;

// the `unsafe impl {}` block is used to declare that a programmer has checked that the contracts the compiler can't check have been upheld inside it
unsafe impl UnsafeSuperpowers for Foo {
    fn raw_pointers() {
        let mut num = 5;

        // creating pointers is not unsafe
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // similarly to `unsafe impl` block the `unsafe {}` block is used to mark that the contracts inside it are not violated
        unsafe {
            // only dereferencing is
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    fn unsafe_functions() {
        unsafe {
            // `unsafe` functions must be called in the unsafe context
            dangerous();
            // so must external functions
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    fn statics() {
        add_to_count(3);

        // accessing a `static` is unsafe too
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    fn union_fields() {
        let mut u = MyUnion { f1: 1 };
        u.f2 = 2.0; // writing to `union` fields is safe
        let f2 = unsafe { u.f2 }; // reading from a `union` field is unsafe
        println!("f2: {}", f2);
    }
}

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

// the `unsafe` keyword in the context of `unsafe fn` is also used to declare the existence of contracts the compiler can't verify, similarly to `unsafe trait`
unsafe fn dangerous() {
    println!("I am very dangerous!");
}

// Rust allows calling functions written in other languages through so-called "Foreign Function Interface"
extern "C" {
    fn abs(x: i32) -> i32;
}

fn add_to_count(inc: u32) {
    // modifying a `static` variable is unsafe
    unsafe {
        COUNTER += inc;
    }
}
