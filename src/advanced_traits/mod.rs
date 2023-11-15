mod trait_bounds;

#[allow(dead_code)]


fn main() {
    println!("Quadruple of 2_i32 is {}", trait_bounds::quadruple(2));
    println!("Quadruple of 2_i32 is {}", trait_bounds::quadruple("2".to_owned()));
}