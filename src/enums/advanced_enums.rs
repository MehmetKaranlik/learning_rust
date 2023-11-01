enum _AdvancedEnum {
    _X(i32),
    _Y { a: &'static str, b: i32 },
}

trait ReturnValue {
    fn return_value(&self) -> i32;
}

/// We can implement traits on enums

impl ReturnValue for _AdvancedEnum {
    fn return_value(&self) -> i32 {
        return match &self {
            _AdvancedEnum::_X(..) => 0,
            _AdvancedEnum::_Y { b, .. } => *b,
        };
    }
}

/// We can also implement methods on as well
impl _AdvancedEnum {
    fn _return_name(&self) -> String {
        return match &self {
            _AdvancedEnum::_X(i) => i.to_string(),
            _AdvancedEnum::_Y { a, .. } => a.to_string(),
        };
    }
}
