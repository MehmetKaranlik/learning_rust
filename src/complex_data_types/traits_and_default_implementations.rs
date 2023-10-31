/// Traits are kind-a interface like structure that shares common behaviour of multiple structures
/// Traits can have methods,types and const parameters.
/// If traits method's is given a body in trait declaration it carries its default value.
/// Unless its overridden.

trait Person {
    type T;
    const X: i32 = 1;
    fn breath_in(&self);
    fn breath_out(&self) { println!("Breath Out!") }
}

/// At below polymorphism is shown over trait
pub fn traits() {
    let student: Student = Student {};
    let worker: Worker = Worker {};
    _static_dispatch(&student);
    _static_dispatch(&worker);
}


fn _static_dispatch<T: Person>(person: &T) {
    person.breath_in();
    person.breath_out();
    /// god knows why you cant use person.X
    /// only by T::X
    println!("{}", T::X);
}


struct Student;

struct Worker;

impl Person for Student {
    type T = i32;
    const X: i32 = 2;

    fn breath_in(&self) { println!("Breath In!") }
}

impl Person for Worker {
    type T = String;
    fn breath_in(&self) { println!("Breath In!") }
}

