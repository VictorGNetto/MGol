use std::fs::File;
use std::io::Write;

pub struct ObjFile {
    content: Vec<String>
}

impl ObjFile {
    pub fn new() -> ObjFile {
        ObjFile {
            content: Vec::new()
        }
    }

    pub fn print(&mut self, s: String) {
        self.content.push(s);
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

        for line in &self.content {
            write!(file, "{}", line);
        }

        writeln!(file, "    return 0;");
        writeln!(file, "}}");
    }
}