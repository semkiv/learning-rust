fn main() {
    scalar_types();
    compound_types();
}

fn scalar_types() {
    integers();
    floating_point();
    booleans();
    characters();
}

fn compound_types() {
    tuples();
    arrays();
}

fn integers() {
    let dec_representation = 97u8; // integers are `i32` by default; an optional type suffix is allowed; valid suffixes are `i8`, `i16`, `i32`, `i64`, `isize` and their unsigned counterparts (`u*`)
    let hex_representation = 0x61; // prefix `0x` signifies hexadecimal notation
    let oct_representation = 0o141; // prefix `0o` signifies octal notation
    let bin_representation = 0b0000_0000_0110_0001; // prefix `0b` signifies binary notation; optional separator `_` is alowed for numeric literals
    let byte_representation = b'a'; // prefix `b` signifies byte notation, applicable to `u8` only

    assert_eq!(dec_representation, hex_representation);
    assert_eq!(hex_representation, oct_representation);
    assert_eq!(oct_representation, bin_representation);
    assert_eq!(bin_representation, byte_representation);

    overflow();
}

fn floating_point() {
    let double = 100_500.000_1; // floating point numbers are `f64` (double precision) by default; separators are allowed
    let float = 0.42f32; // single precision can be specified through type suffix (or type annotation)
    assert!(float < double);
}

fn booleans() {
    let boolean = true;
    assert!(boolean);
}

fn characters() {
    let character = '😻'; // `char`s literals are specified in single quotes; these represent Unicode Scalar Values (32-bit Unicode "character")
    assert_eq!(character, '\u{1F63B}'); // `char`s can be specified as Unicode code points
}

fn tuples() {
    let transparent_navy: (u8, u8, u8, f64) = (0, 0, 128, 0.5); // should normally be a `struct`, tuple only as an example; alternative form would be `let transparent_navy = (0u8, 0u8, 128u8, 0.5)`; without explicit specifiaction type would be inferred to `(i32, i32, i32, f64)`
    let (_, _, blue_component, _) = transparent_navy; // destructuring the tuple; since we only care about the blue component, the others are placeholders
    assert_eq!(blue_component, 128);
    assert_eq!(transparent_navy.1, 0); // tuple members can be accessed by their index without destructuring
}

fn arrays() {
    let arr = [1, 2, 3]; // an array
    assert_eq!(arr[0], 1); // accessing array elements
    let arr: [u8; 5] = [42; 5]; // array with the explicitly specialized type initialized using the form [init-expr; size]; alternative form would be simply `let arr = [42u8, 5]`
    assert_eq!(arr[4], 42);
}

fn overflow() {
    let at_limit = 127i8; // Rust uses two's complement and guarantees that `iN` can have values from (-(2 ^ ( N - 1))) to (2 ^ (N - 1) - 1) and `uN` - from 0 to (2 ^ N - 1)

    // at_limit += 1; // this panics in debug mode and two’s complement wraps in release mode; relying on implicit wrapping is considered bad practice - there's a set of explicit alternatives

    let result = at_limit.wrapping_add(1); // `wrapping_*` family of members always wrap
    assert_eq!(result, -128);

    let result = at_limit.checked_add(1); // `checked_*` family of members returns `Some(T)` if the overflow did not happen or `None` otherwise
    assert_eq!(result, None);

    let (result, has_overflown) = at_limit.overflowing_add(1); // `overflowing_*` family of members returns the wrapped value and a boolean indicating if the overflow happened
    assert_eq!(result, -128);
    assert_eq!(has_overflown, true);

    let result = at_limit.saturating_add(1); // `saturating_*` family of members saturate at the value's minimum or maximum values
    assert_eq!(result, 127);
}
