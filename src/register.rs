use std::collections::HashMap;

pub struct Register{
    name: String,
    size: u64,
    value: i64,
    flags: HashMap<String, bool>
}

impl Register{
    pub fn new(name: String) -> Self{
        Register { name, size: 64, value: 0, flags: HashMap::from([
            ("Zero".to_string(), true), 
            ("Negative".to_string(), false), 
            ("Overflow".to_string(), false), 
            ("Carry".to_string(), false)]) 
        }
    }

    pub fn write(&mut self, valor: i64){
        if (size_of_val(&valor) as u64) < self.size{
            self.value = valor;

            self.set_flag("Zero".to_string(), self.value == 0);
            self.set_flag("Negative".to_string(), self.value < 0);
        } else {
            println!("Error: el valor el demasiado grande");
        }
    }

    pub fn read(&self) -> i64{
        self.value
    }

    pub fn set_flag(&mut self, flag: String, estado: bool){
        if self.flags.contains_key(&flag){
            self.flags.insert(flag, estado);
        } else {
            println!("Nombre de flag no válido");
        }
    }

    pub fn get_flag(&self, flag: String) -> bool{
        if self.flags.contains_key(&flag){
            *self.flags.get(&flag).unwrap()
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String{
        let mut registro = String::new();
        registro += "Registro: ";
        registro += &self.name as &str;
        registro += "\t Valor: ";
        registro += &self.read().to_string() as &str;
        registro += "\t Flags: ";
        registro +=  &self.true_flags_to_string() as &str;
        
        registro
    }

    pub fn true_flags_to_string(&self) -> String{
        let mut flags = String::new();

        self.flags.iter().for_each(|f| {
            if self.get_flag(f.0.to_string()) {
                flags += f.0;
                flags.push(' ');
            }
        });

        if flags.len() == 0{
            return "None".to_string()
        }

        flags
    }
}

impl Clone for Register{
    fn clone(&self) -> Self {
        Register { name: self.name.clone(), size: self.size, value: self.value, flags: self.flags.clone() }
    }
}