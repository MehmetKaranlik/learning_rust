mod algorithms;
mod complex_data_types;
mod control_flow;
mod data_structures;
mod enums;
mod functions;
mod generics;
mod hashmaps;
mod iterators;
mod ownership;
mod smart_pointers;
mod variables;

fn main() {
    // variables::basic_variables::variables();
    // variables::shadowing::shadowing();
    // variables::compound_variables::compound_variables();
    // functions::basic_functions::basic_function();
    // ownership::ownership_references_in_functions::ownership_references_in_functions();
    // control_flow::control_flow::control_flow();
    // data_structures::stack::stack();
    // complex_data_types::complex_data_types::structs();
    // complex_data_types::traits_and_default_implementations::traits()
    // enums::enums::enums()
    // hashmaps::hashmaps::hash_maps()
    // iterators::iterators::iterators()
    // smart_pointers::smart_pointers::smart_pointers()
    let x: Vec<i32> = [].to_vec();
    x.iter();
    std::thread::park();
}
