use std::collections::HashMap;

pub struct Registro{
    nombre: String,
    tamaño: u64,
    valor: i64,
    flags: HashMap<String, bool>
}

impl Registro{
    pub fn nuevo(nombre: String) -> Self{
        Registro { nombre, tamaño: 64, valor: 0, flags: HashMap::from([("Zero".to_string(), true), ("Negative".to_string(), false), ("Overflow".to_string(), false), ("Carry".to_string(), false)]) }
    }

    pub fn write(&mut self, valor: i64){
        if (size_of_val(&valor) as u64) < self.tamaño{
            self.valor = valor;

            self.set_flag("Zero".to_string(), self.valor == 0);
            self.set_flag("Negative".to_string(), self.valor < 0);
        } else {
            println!("Error: el valor el demasiado grande");
        }
    }

    pub fn read(&self) -> i64{
        self.valor
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
        registro += &self.nombre as &str;
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

impl Clone for Registro{
    fn clone(&self) -> Self {
        Registro { nombre: self.nombre.clone(), tamaño: self.tamaño, valor: self.valor, flags: self.flags.clone() }
    }
}