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

    // TODO: 다음 함수 작성
}
