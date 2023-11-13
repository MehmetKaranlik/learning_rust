mod trait_bounds;
mod associated_types;


fn main() {
    println!("Quadruple of 2_i32 is {}", trait_bounds::quadruple(2));
    println!("Quadruple of 2_i32 is {}", trait_bounds::quadruple("2".to_owned()));
}