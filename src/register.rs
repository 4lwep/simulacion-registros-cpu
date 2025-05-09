use std::{collections::HashMap, hash::Hash};

pub struct Register{
    pub name: String,
    size: u64,
    value: i64,
}

impl Register{
    pub fn new(name: String) -> Self{
        Register { name, size: 64, value: 0 }
    }

    pub fn write(&mut self, valor: i64){
        if (size_of_val(&valor) as u64) < self.size{
            self.value = valor;
        } else {
            println!("Error: el valor el demasiado grande");
        }
    }

    pub fn read(&self) -> i64{
        self.value
    }

    pub fn to_string(&self) -> String{
        let mut registro = String::new();
        registro += "Registro: ";
        registro += &self.name as &str;
        registro += "\t Valor: ";
        registro += &self.read().to_string() as &str;
        
        registro
    }
}

impl Clone for Register{
    fn clone(&self) -> Self {
        Register { name: self.name.clone(), size: self.size, value: self.value }
    }
}

impl PartialEq for Register{
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Register {
    
}

impl Hash for Register{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub struct CPSR{
    flags: HashMap<String, bool>
}

impl CPSR{
    pub fn new() -> Self{
        CPSR { flags: HashMap::from([
            ("Zero".to_string(), true), 
            ("Negative".to_string(), false), 
            ("Overflow".to_string(), false), 
            ("Carry".to_string(), false)])  }
    }

    pub fn set_flags(&mut self, value: &Register){
        let value = value.read();

        self.flags.insert("Zero".to_string(), value == 0);
        self.flags.insert("Negative".to_string(), value < 0);
    }

    pub fn flags_to_string(&self) -> String{
        let mut flags = String::new();
        println!("\n----FLAGS----");
        self.flags.keys().for_each(|k| {
            flags += k;
            flags += ": ";
            flags += &self.flags.get(k).unwrap().to_string() as &str;
            flags.push('\n');
        });
        flags
    }
}