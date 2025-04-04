use crate::register::*;

pub fn mov<T: RegisterValue>(registro: &mut Register, valor: &T){
    registro.write(valor.read());
}

pub fn mvn<T: RegisterValue>(registro: &mut Register, valor: &T){
    registro.write(!valor.read());
}
