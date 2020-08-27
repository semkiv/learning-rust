#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
    _Arizona,
    _Arkansas,
    _California,
    _Colorado,
    _Connecticut,
    _Delaware,
    _Florida,
    _Georgia,
    _Hawaii,
    _Idaho,
    _Illinois,
    _Indiana,
    _Iowa,
    _Kansas,
    _Kentucky,
    _Louisiana,
    _Maine,
    _Maryland,
    _Massachusetts,
    _Michigan,
    _Minnesota,
    _Mississippi,
    _Missouri,
    _Montana,
    _Nebraska,
    _Nevada,
    _NewHampshire,
    _NewJersey,
    _NewMexico,
    _NewYork,
    _NorthCarolina,
    _NorthDakota,
    _Ohio,
    _Oklahoma,
    _Oregon,
    _Pennsylvania,
    _RhodeIsland,
    _SouthCarolina,
    _SouthDakota,
    _Tennessee,
    _Texas,
    _Utah,
    _Vermont,
    _Virginia,
    _Washington,
    _WestVirginia,
    _Wisconsin,
    _Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => { // `match` arms are expressions and and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

fn main() {
    println!("{}", Coin::Penny.value_in_cents());
    println!("{}", Coin::Nickel.value_in_cents());
    println!("{}", Coin::Dime.value_in_cents());
    println!("{}", Coin::Quarter(UsState::Alabama).value_in_cents());
}
