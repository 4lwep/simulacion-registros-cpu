use register::Register;
use crate::operations::arithmetic::*;
use crate::operations::movement::*;

mod operations;
mod register;

fn main() {
    let mut r0: Register = Register::new("r0".to_string());
    let mut r1: Register = Register::new("r1".to_string());
    let mut r2: Register = Register::new("r2".to_string());
    let mut r3: Register = Register::new("r3".to_string());
    let mut r4: Register = Register::new("r4".to_string());
    let mut r5: Register = Register::new("r5".to_string());
    let mut r6: Register = Register::new("r6".to_string());
    let mut r7: Register = Register::new("r7".to_string());
    let mut r8: Register = Register::new("r8".to_string());
    let mut r9: Register = Register::new("r9".to_string());
    let mut r10: Register = Register::new("r10".to_string());
    let mut r11: Register = Register::new("r11".to_string());
    let mut r12: Register = Register::new("r12".to_string());
    let mut r13: Register = Register::new("r13".to_string());
    let mut r14: Register = Register::new("r14".to_string());
    let mut r15: Register = Register::new("r15".to_string());

    mov(&mut r0, &12);
    mov(&mut r1, &32);

    add(&mut r0, &r1);
    sub(&mut r0, &r1);
    mul(&mut r0, &r1);

    mov(&mut r3, &r1);

    let ayuda_r3 = r3.clone();
    mov(&mut r3, &ayuda_r3);
    mov(&mut r2, &5);
    mvn(&mut r4, &r2);

    let ayuda_r4 = r4.clone();
    mvn(&mut r4, &ayuda_r4);

    let ayuda_r5 = r5.clone();
    add(&mut r1, &r2);
    add(&mut r5, &ayuda_r5);

    println!("{}", r0.to_string());
    println!("{}", r1.to_string());
    println!("{}", r2.to_string());
    println!("{}", r3.to_string());
    println!("{}", r4.to_string());
    println!("{}", r5.to_string());
    println!("{}", r6.to_string());
    println!("{}", r7.to_string());
    println!("{}", r8.to_string());
    println!("{}", r9.to_string());
    println!("{}", r10.to_string());
    println!("{}", r11.to_string());
    println!("{}", r12.to_string());
    println!("{}", r13.to_string());
    println!("{}", r14.to_string());
    println!("{}", r15.to_string());
}