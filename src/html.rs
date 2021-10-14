struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while(&mut self, test: F) -> String 
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.eof() && test(self.next_char()) {
                result.push(self.consume_char());
            }
            return result;

        }
    
    fn consume_whitespace(&mut self) {
        self.consume_while(CharExt::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            "a".."z" | "A".."Z" | "0".."9" => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            "<" => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != "<"))
    }

    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_char() == "<");
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == ">");
        let children = self.parse_nodes();

        
    }
}