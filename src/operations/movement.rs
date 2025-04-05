use crate::register::*;

pub fn mov(registro: &mut Register, valor: &i64){
    registro.write(*valor);
}

pub fn mvn(registro: &mut Register, valor: &i64){
    registro.write(!valor);
}
