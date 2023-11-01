/// [Struct]
/// There is nothing new, structs unlike classes cant have methods
/// Inside their body. just some params and values.

pub fn _structs() {
    let basic_struct = _BasicStruct {
        param: "Value".to_string(),
    };
    basic_struct._do_something();
    basic_struct._new_instance();
    println!("{}", basic_struct.param);
}

struct _BasicStruct {
    param: String,
}

/// Impl block is a simply a way to register methods on a [Struct]
/// At below we are gonna implement some methods on [BasicStruct]
/// To assign method to structure we should take [&self] as argument.
/// [self] means instance itself, [Self] means type.
impl _BasicStruct {
    fn _do_something(&self) {}
    fn _new_instance(&self) -> Self {
        Self {
            param: "Value".to_string(),
        }
    }
}
