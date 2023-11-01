/// In rust enumerations have few variants

pub fn _enums() {
    let mut _a: _BasicEnum = _BasicEnum::Z;
    _a = _BasicEnum::X;

    let mut _b: _EnumWithParams = _EnumWithParams::Y {
        name: "Value".to_string(),
    };
    _b = _EnumWithParams::X("Value".to_string(), 3.12)
}

enum _BasicEnum {
    X,
    Z,
}

///An enum where no constructors contain fields are called A field-less enum.
/// For example, this is a field-less enum:
enum _FieldlessEnum {
    Tuple(i32),
    Struct { a: u8, b: i32 },
    Unit,
}

///If A field-less enum only contains unit variants,
/// the enum is called an unit-only enum. For example:
enum _UnitEnum {
    A = 1,
    B = 2,
}

///Enum constructors can have either named or unnamed fields:
enum _EnumWithParams {
    /// Unnamed parameters (Enum variant)
    X(String, f64),
    /// Named parameters (Struct-Like enum variant)
    Y { name: String },
}
