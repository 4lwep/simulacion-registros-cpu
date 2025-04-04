use crate::Register;
use super::movement::mov;


pub fn add(registro_primero: &mut Register, registro_segundo: &Register){
    mov(registro_primero, &(registro_primero.read() + registro_segundo.read())) 
}

pub fn sub(registro_primero: &mut Register, registro_segundo: &Register){
    mov(registro_primero, &(registro_primero.read() - registro_segundo.read())) 
}

pub fn mul(registro_primero: &mut Register, registro_segundo: &Register){
    mov(registro_primero, &(registro_primero.read() * registro_segundo.read())) 
}