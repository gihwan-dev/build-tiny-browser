#[derive(Debug)]
struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
struct SimpleSelector {
    tag_name: Option(String),
    id: Option(String),
    class: Vec<String>,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // 더 많의 값 추가
}

#[derive(Debug, Clone, PartialEq)]
enum Unit {
    Px,
}

#[derive(Debug, Clone, PartialEq)]
struct Color {
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
