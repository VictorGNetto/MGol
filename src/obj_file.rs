use std::fs::File;
use std::io::Write;

pub enum TempVarType {
    Int,
    Real,
}

pub struct ObjFile {
    temp_vars: Vec<TempVarType>,
    content: Vec<String>,
}

impl ObjFile {
    pub fn new() -> ObjFile {
        ObjFile {
            temp_vars: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn print(&mut self, s: String) {
        self.content.push(s);
    }

    pub fn add_temp_var(&mut self, temp_var_type: TempVarType) {
        self.temp_vars.push(temp_var_type);
    }

    pub fn create(&self) {
        let mut file = match File::create("./PROGRAMA.c") {
            Err(_) => panic!("Não foi possível criar o código objeto PROGRAMA.c"),
            Ok(file) => file,
        };

        writeln!(file, "#include <stdio.h>");
        writeln!(file);
        writeln!(file, "typedef char literal[256];");
        writeln!(file, "typedef int inteiro;");
        writeln!(file, "typedef double real;");
        writeln!(file);
        writeln!(file, "void main(void)");
        writeln!(file, "{{");
        if self.temp_vars.len() > 0 {
            writeln!(file, "    /*----Variaveis temporarias----*/");
            for i in 0..self.temp_vars.len() {
                let temp_var_type = &self.temp_vars[i];
                match temp_var_type {
                    TempVarType::Int => {
                        writeln!(file, "    inteiro T{}", i);
                    }
                    TempVarType::Real => {
                        writeln!(file, "    real T{}", i);
                    }
                }
            }
            writeln!(file, "    /*------------------------------*/");
        }

        for line in &self.content {
            write!(file, "{}", line);
        }

        writeln!(file, "    return 0;");
        writeln!(file, "}}");
    }
}
