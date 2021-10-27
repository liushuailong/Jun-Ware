#[derive(Debug)]
pub struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}


#[derive(Debug)]
pub struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Option<String>,
}

#[derive(Debug)]
pub struct Declaration {
    name: String,
    value: Value,
}


#[derive(Debug, Clone, ParticalEq)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, Clone, ParticalEq)]
pub impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, ParticalEq)]
pub enum Unit {
    Px,
}

#[derive(Debug, Clone, ParticalEq, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Copy for Color {}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0, 
        input: source,
    };
    Stylesheet {
        rules: parser.parse_rules(),
    }

}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {


    fn parse_rules(&mut self) {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() { break };
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarationsa: self.parse_declarations(), 
        }
    }
    fn parse_simple_selector(&mut self) -> SampleSelector {
        let mut selector = SampleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                },
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                },
                '*' => {
                    self.consume_char();
                },
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                },
                _ => break,
            }
        }
        return selector;
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                },
                '{' => {
                    break
                },
                c => {
                    panic!("Unexpected character {} in selector list", c)
                },
            }
        }
        selector.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selector;
    }


}
