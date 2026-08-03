use std::{fs, io::Read};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Node {
    Tag(Tag),
    VoidTag(VoidTag),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct VoidTag {
    pub name: String,
    pub attributes: Vec<Attribute>,
}
impl Node {
    pub fn to_html(&self) -> String {
        match self {
            Node::Tag(tag) => tag.to_html(),
            Node::VoidTag(vt) => vt.to_html(),
            Node::Text(text) => text.clone(),
        }
    }
}

impl Tag {
    pub fn to_html(&self) -> String {
        let attrs = format_attributes(&self.attributes);
        let children: Vec<String> = self.children.iter().map(|c| c.to_html()).collect();
        format!(
            "<{}{}>{}</{}>",
            self.name,
            attrs,
            children.join(" "),
            self.name
        )
    }
}

impl VoidTag {
    pub fn to_html(&self) -> String {
        let attrs = format_attributes(&self.attributes);
        format!("<{}{}>", self.name, attrs)
    }
}

fn format_attributes(attrs: &[Attribute]) -> String {
    if attrs.is_empty() {
        String::new()
    } else {
        let formatted: Vec<String> = attrs
            .iter()
            .map(|a| format!("{}=\"{}\"", a.name, a.value))
            .collect();
        format!(" {}", formatted.join(" "))
    }
}

pub struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn parse_node(&mut self) -> Option<Node> {
        self.skip_whitespace();
        let c = self.peek()?;

        if c == '"' {
            Some(Node::Text(self.parse_string_literal()))
        } else if c == '|' {
            Some(Node::VoidTag(self.parse_void_tag()))
        } else if c.is_alphanumeric() || c == '_' {
            Some(Node::Tag(self.parse_tag()))
        } else {
            None
        }
    }

    fn parse_string_literal(&mut self) -> String {
        self.advance();
        let mut content = String::new();
        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            content.push(c);
        }
        content
    }

    fn parse_tag(&mut self) -> Tag {
        let name = self.parse_identifier();
        self.skip_whitespace();

        let attributes = if self.peek() == Some('(') {
            self.parse_attributes()
        } else {
            Vec::new()
        };

        self.skip_whitespace();
        let mut children = Vec::new();

        if self.peek() == Some('{') {
            self.advance();
            loop {
                self.skip_whitespace();
                if self.peek() == Some('}') || self.peek().is_none() {
                    break;
                }
                if let Some(child) = self.parse_node() {
                    children.push(child);
                }
            }
            if self.peek() == Some('}') {
                self.advance();
            }
        }

        Tag {
            name,
            attributes,
            children,
        }
    }

    fn parse_void_tag(&mut self) -> VoidTag {
        self.advance();
        let name = self.parse_identifier();
        self.skip_whitespace();

        let attributes = if self.peek() == Some('(') {
            self.parse_attributes()
        } else {
            Vec::new()
        };

        self.skip_whitespace();
        if self.peek() == Some('|') {
            self.advance();
        }

        VoidTag { name, attributes }
    }

    fn parse_attributes(&mut self) -> Vec<Attribute> {
        self.advance();
        let mut attrs = Vec::new();

        loop {
            self.skip_whitespace();
            if self.peek() == Some(')') || self.peek().is_none() {
                break;
            }

            let name = self.parse_identifier();
            self.skip_whitespace();

            let mut value = String::new();
            if self.peek() == Some('=') {
                self.advance();
                self.skip_whitespace();
                if self.peek() == Some('"') {
                    value = self.parse_string_literal();
                }
            }

            attrs.push(Attribute { name, value });

            self.skip_whitespace();
            if self.peek() == Some(',') {
                self.advance();
            }
        }

        if self.peek() == Some(')') {
            self.advance();
        }

        attrs
    }

    fn parse_identifier(&mut self) -> String {
        let mut id = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }
        id
    }
}

fn main() {
    let mut buf = String::new();
    let mut source = fs::File::open("test.smkl").expect("erro lendo");
    source.read_to_string(&mut buf);
    let mut parser = Parser::new(&buf);
    if let Some(ast) = parser.parse_node() {
        println!("{}", ast.to_html());
    }
}
