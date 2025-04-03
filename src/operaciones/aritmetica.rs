use crate::Registro;
use super::movimiento::mov;


pub fn add(registro_primero: &mut Registro, registro_segundo: &Registro){
    mov(registro_primero, &(registro_primero.read() + registro_segundo.read())) 
}

pub fn sub(registro_primero: &mut Registro, registro_segundo: &Registro){
    mov(registro_primero, &(registro_primero.read() - registro_segundo.read())) 
}

pub fn mul(registro_primero: &mut Registro, registro_segundo: &Registro){
    mov(registro_primero, &(registro_primero.read() * registro_segundo.read())) 
}
