#[allow(dead_code)]
pub trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        *self * 2
    }
}

impl Double for String {
    fn double(&self) -> Self {
        self.repeat(2)
    }
}

#[allow(dead_code)]
pub fn quadruple<T: Double>(t: T) -> T {
    t.double().double()
}