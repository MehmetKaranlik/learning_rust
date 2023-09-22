/// In rust enumerations have few variants




pub fn enums() {
    let mut a: BasicEnum = BasicEnum::z;
    a = BasicEnum::x;

    let mut b: EnumWithParams = EnumWithParams::y { name: "Value".to_string() };
    b = EnumWithParams::x("Value".to_string(), 3.12)
}


enum BasicEnum {
    x,
    z,
}


///An enum where no constructors contain fields are called a field-less enum.
/// For example, this is a fieldless enum:
enum FieldlessEnum {
    Tupple(i32),
    Struct {
        a: u8,
        b: i32,
    },
    Unit,
}


///If a field-less enum only contains unit variants,
/// the enum is called an unit-only enum. For example:
enum UnitEnum {
    a = 1,
    b = 2,
}


///Enum constructors can have either named or unnamed fields:
enum EnumWithParams {
    /// Unnamed parameters (Enum variant)
    x(String, f64),
    /// Named parameters (Struct-Like enum variant)
    y { name: String },
}




