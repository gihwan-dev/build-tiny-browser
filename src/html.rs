use std::collections::HashMap;

use crate::dom;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // 현재 문자를 소비하지 않고 읽는다.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // 다음 문자가 주어진 string 으로 시작하는지 확인한다.
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // 만약 현재 위치에서 문자열 s 가 시작한다면 소비한다.
    // 그렇지 않으면 panic 을 일으킨다.
    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {} but it was found", s, self.pos);
        }
    }

    // 모든 입력을 다 읽었다면 true 를 반환한다.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // pos 를 다음 문자의 위치로 옮기고 문자를 반환한다.
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        let children = self.parse_nodes();

        self.expect("<");
        self.expect(&tag_name);
        self.expect(">");

        dom::elem(tag_name, attrs, children)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        value
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();

            if self.next_char() == '>' {
                break;
            }

            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }

        attributes
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();

        loop {
            self.consume_whitespace();

            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }

        nodes
    }
}

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    if nodes.len() == 1 {
        nodes.remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}
