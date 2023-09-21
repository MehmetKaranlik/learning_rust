///Functions, type aliases, structs, enumerations, unions, traits,
/// and implementations may be parameterized by types,
/// constants, and lifetimes.
/// These parameters are listed in angle brackets (<...>),
/// usually immediately after the name of the item and before its definition.
/// For implementations, which don't have a name, they come directly after impl.
/// The order of generic parameters is restricted to lifetime parameters
/// and then type and const parameters intermixed.

struct X;

trait Traits {}

impl Traits for X {}

pub fn generics() {}

fn basic_generic_func<T>(value: T) {}

/// const generics
/// The only allowed types of const parameters are u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char and bool.
fn generic_func_with_restraint<const T: i32>() {}

/// Where clause generally used for if generic is amount is higher, it helps for readability.
fn generics_for_polymorphism<T, R, Z, X>() where
    T: Traits,
    R: Traits,
    Z: Traits,
    X: Traits,
{}

struct Bearer<T: Traits>;

struct Bearer2<T> where T: Traits;
