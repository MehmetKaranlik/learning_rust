
/// [Struct]
/// There is nothing new, structs unlike classes cant have methods
/// Inside their body. just some params and values.

pub fn structs() {
    let basic_struct = BasicStruct{param: "Value".to_string()};
    basic_struct.do_something();
    basic_struct.new_instance();
    println!("{}", basic_struct.param);
}

struct BasicStruct {
    param : String
}

/// Impl block is a simply a way to register methods on a [Struct]
/// At below we are gonna implement some methods on [BasicStruct]
/// To assign method to structure we should take [&self] as argument.
/// [self] means instance itself, [Self] means type.
impl  BasicStruct {
    fn do_something(&self){}
     fn new_instance(&self) -> Self  {
         Self { param: "Value".to_string() }
     }
}

