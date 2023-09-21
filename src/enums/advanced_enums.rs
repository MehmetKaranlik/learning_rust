enum AdvancedEnum {
    X(str),
    Y {
        a: str,
        b: i32,
    },
}


trait ReturnValue {
    fn return_value(&self) -> i32;
}

/// We can implement traits on enums

impl ReturnValue for AdvancedEnum {
    fn return_value(&self) -> i32 {
        return match &self {
            AdvancedEnum::x(..) => 0,
            AdvancedEnum::y { b, .. } => b,
        };
    }
}

/// We can also implement methods on as well
impl AdvancedEnum {
    fn return_name(&self) -> String {
        return match &self {
            AdvancedEnum::X(str) => str.to_string(),
            AdvancedEnum::Y { a, .. } => a.to_string(),
        };
    }
}