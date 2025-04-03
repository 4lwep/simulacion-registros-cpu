use crate::registro::*;
pub trait ValorRegistro{
    fn read(&self) -> i64;
}

impl ValorRegistro for Registro{
    fn read(&self) -> i64{
        self.read()
    }
}

impl ValorRegistro for i64{
    fn read(&self) -> i64{
        *self
    }
}

pub fn mov<T: ValorRegistro>(registro: &mut Registro, valor: &T){
    registro.write(valor.read());
}

pub fn mvn<T: ValorRegistro>(registro: &mut Registro, valor: &T){
    registro.write(!valor.read());
}
