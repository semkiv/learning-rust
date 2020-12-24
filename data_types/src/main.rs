fn main() {
    // scalar types
    let _byte_var: i8 = -64; // signed 8-bit
    let _unsigned_byte_var = b'A'; // unsigned 8-bit (declared using a byte literal)
    let _short_var = -1024i16; // signed 16-bit (declared using a suffix type)
    let _unsigned_short_var: u16 = 0xFFFF; // unsigned 16-bit (declared as a hex number)
    let int_var = 0o7777_7777; // signed 32-bit (default choice for integers, declared as a oct number)
    let unsigned_int_var = 0b11111111_11111111_11111111_00000000u32; // unsigned 32-bit (declared as a binary number using a suffix type)
    let _long_var = -100500i64; // signed 64-bit (declared using a suffix type)
    let _unsigned_long_var: u64 = 100500; // unsigned 64-bit
    let _long_long_var: i128 = -100500100500; // signed 128-bit
    let _unsigned_long_long_var = 100500100500u128; // unsigned 128-bit (declared using a suffix type)
    let _arch_size_var: isize = -64; // signed arch size (32 or 64 bits)
    let _unsigned_arch_size_var = 64usize; // signed arch size (32 or 64 bits, declared using a suffix type)
    let float_var = 0.42f32; // 32-bit single precision float (declared using a suffix type)
    let _double_var: f64 = 0.42; // 64-bit double precision float (default choice for floats)
    let _bool_var = false; // 8-bit boolean type
    let char_type = '😻'; // 32-bit Unicode character

    //compound types
    let tuple_var_1 = (unsigned_int_var, float_var, char_type); // tuple type with underlying types being inferred
    let (_, _, z) = tuple_var_1; // destructuring the tuple
    println!("{}", z);

    let tuple_var_2 : (f64, f64) = (f64::from(float_var), f64::from(int_var)); // tuple type with underlying types explicitly specified; implicit conversions are forbidden
    println!("{}", tuple_var_2.1); // accessing tuple members without destructuring

    let arr = [1, 2, 3]; // array
    println!("{}", arr[0]); // accessing array elements
    let arr: [u8; 5] = [0; 5]; // array with the explicitly specialized type initialized using the form [init-expr; size]; making use of shadowing
    println!("{}", arr[2]); // accessing array elements
}
