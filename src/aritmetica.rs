use crate::Registro;


pub fn add(registro_primero: &Registro, registro_segundo: &Registro) -> i64{
    registro_primero.read() + registro_segundo.read()
}

pub fn sub(registro_primero: &Registro, registro_segundo: &Registro) -> i64{
    registro_primero.read() - registro_segundo.read()
}

pub fn mul(registro_primero: &Registro, registro_segundo: &Registro) -> i64{
    registro_primero.read() * registro_segundo.read()
}
