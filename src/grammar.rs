pub struct AlphabetItem {
    pub text: String,
    pub terminal: bool,
}

pub struct GrammarRule {
    left: AlphabetItem,
    right: Vec<AlphabetItem>,
}

pub struct Grammar {
    rules: Vec<GrammarRule>,
}

impl Grammar {
    pub fn new() -> Grammar {
        let mut grammar = Grammar { rules: vec![] };
        grammar.init_rules();

        grammar
    }

    pub fn get_rule(&self, index: usize) -> &GrammarRule {
        &self.rules[index - 1]
    }

    pub fn show(&self) {
        for n in 0..self.rules.len() {
            let left = &self.rules[n].left;
            let right = &self.rules[n].right;
            println!(
                "{}. {} -> {}",
                n + 1,
                left.text,
                right
                    .iter()
                    .map(|item| String::from(&item.text))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }

    fn init_rules(&mut self) {
        self.add_grammar_rule("P'", "P");
        self.add_grammar_rule("P", "inicio V A");
        self.add_grammar_rule("V", "varincio LV");
        self.add_grammar_rule("LV", "D LV");
        self.add_grammar_rule("LV", "varfim pt_v");
        self.add_grammar_rule("D", "TIPO L pt_v");
        self.add_grammar_rule("L", "id");
        self.add_grammar_rule("TIPO", "inteiro");
        self.add_grammar_rule("TIPO", "real");
        self.add_grammar_rule("TIPO", "literal");
        self.add_grammar_rule("A", "ES A");
        self.add_grammar_rule("ES", "leia id pt_v");
        self.add_grammar_rule("ES", "escreva ARG pt_v");
        self.add_grammar_rule("ARG", "lit");
        self.add_grammar_rule("ARG", "num");
        self.add_grammar_rule("ARG", "id");
        self.add_grammar_rule("A", "CMD A");
        self.add_grammar_rule("CMD", "id rcb LD pt_v");
        self.add_grammar_rule("LD", "OPRD opm OPRD");
        self.add_grammar_rule("LD", "OPRD");
        self.add_grammar_rule("OPRD", "id");
        self.add_grammar_rule("OPRD", "num");
        self.add_grammar_rule("A", "COND A");
        self.add_grammar_rule("COND", "CAB CP");
        self.add_grammar_rule("CAB", "se ab_p EXP_R fc_p entao");
        self.add_grammar_rule("EXP_R", "OPRD opr OPRD");
        self.add_grammar_rule("CP", "ES CP");
        self.add_grammar_rule("CP", "CMD CP");
        self.add_grammar_rule("CP", "COND CP");
        self.add_grammar_rule("CP", "fimse");
        self.add_grammar_rule("A", "R A");
        self.add_grammar_rule("R", "CABR CPR");
        self.add_grammar_rule("CABR", "repita ab_p EXP_R fc_p");
        self.add_grammar_rule("CPR", "ES CPR");
        self.add_grammar_rule("CPR", "CMD CPR");
        self.add_grammar_rule("CPR", "COND CPR");
        self.add_grammar_rule("CPR", "fimrepita");
        self.add_grammar_rule("A", "fim");
    }

    fn add_grammar_rule(&mut self, left_str: &str, right_str: &str) {
        let left = AlphabetItem {
            text: String::from(left_str),
            terminal: false,
        };

        let mut right = Vec::new();
        for item in right_str.split_whitespace() {
            right.push(AlphabetItem {
                text: String::from(item),
                terminal: item.to_lowercase().eq(item),
            });
        }

        self.rules.push(GrammarRule { left, right });
    }
}
