use crate::register::*;
pub trait RegisterValue{
    fn read(&self) -> i64;
}

impl RegisterValue for Register{
    fn read(&self) -> i64{
        self.read()
    }
}

impl RegisterValue for i64{
    fn read(&self) -> i64{
        *self
    }
}

pub fn mov<T: RegisterValue>(registro: &mut Register, valor: &T){
    registro.write(valor.read());
}

pub fn mvn<T: RegisterValue>(registro: &mut Register, valor: &T){
    registro.write(!valor.read());
}
