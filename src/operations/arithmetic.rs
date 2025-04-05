use crate::register::*;
use super::movement::mov;


pub fn add(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &i64){
    mov(registro_destino, &(registro_primero.read() + valor_segundo)) 
}

pub fn sub(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &i64){
    mov(registro_destino, &(registro_primero.read() - valor_segundo)) 
}

pub fn mul(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &i64){
    mov(registro_destino, &(registro_primero.read() * valor_segundo)) 
}

pub fn rsb(registro_destino: &mut Register, registro_primero: &Register, valor_segundo: &i64){
    mov(registro_destino, &(valor_segundo - registro_primero.read())) 
}