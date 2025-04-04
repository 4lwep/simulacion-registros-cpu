use crate::register::*;
use super::movement::mov;


pub fn add<T: RegisterValue>(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &T){
    mov(registro_destino, &(registro_primero.read() + valor_segundo.read())) 
}

pub fn sub<T: RegisterValue>(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &T){
    mov(registro_destino, &(registro_primero.read() - valor_segundo.read())) 
}

pub fn mul<T: RegisterValue>(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &T){
    mov(registro_destino, &(registro_primero.read() * valor_segundo.read())) 
}

pub fn rsb<T: RegisterValue>(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &T){
    mov(registro_destino, &(valor_segundo.read() - registro_primero.read())) 
}