use std::env;
use std::fs;

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
    pub fn to_html(&self, indent: usize) -> String {
        match self {
            Node::Tag(tag) => tag.to_html(indent),
            Node::VoidTag(vt) => vt.to_html(indent),
            Node::Text(text) => format!("{}{}", "  ".repeat(indent), text),
        }
    }
}

impl Tag {
    pub fn to_html(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent);
        let attrs = format_attributes(&self.attributes);

        if self.children.len() == 1 {
            if let Node::Text(txt) = &self.children[0] {
                return format!("{}<{}{}>{}</{}>", padding, self.name, attrs, txt, self.name);
            }
        }

        if self.children.is_empty() {
            return format!("{}<{}{}></{}>", padding, self.name, attrs, self.name);
        }

        let children_html: Vec<String> = self
            .children
            .iter()
            .map(|c| c.to_html(indent + 1))
            .collect();

        format!(
            "{}<{}{}>\n{}\n{}</{}>",
            padding,
            self.name,
            attrs,
            children_html.join("\n"),
            padding,
            self.name
        )
    }
}

impl VoidTag {
    pub fn to_html(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent);
        let attrs = format_attributes(&self.attributes);
        format!("{}<{}{}>", padding, self.name, attrs)
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

    pub fn parse_document(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while self.peek().is_some() {
            self.skip_whitespace();
            if self.peek().is_none() {
                break;
            }
            if let Some(node) = self.parse_node() {
                nodes.push(node);
            } else {
                self.advance();
            }
        }
        nodes
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

pub fn transpile(smkl_code: &str) -> String {
    let mut parser = Parser::new(smkl_code);
    let ast_nodes = parser.parse_document();

    ast_nodes
        .iter()
        .map(|node| node.to_html(0))
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Uso: {} <origem.smkl> [destino.html]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() == 3 {
        args[2].as_str()
    } else {
        "output.html"
    };

    let source = fs::read_to_string(input_path)
        .unwrap_or_else(|err| panic!("Erro ao ler o arquivo '{}': {}", input_path, err));

    let html_output = transpile(&source);

    fs::write(output_path, html_output)
        .unwrap_or_else(|err| panic!("Erro ao salvar em '{}': {}", output_path, err));

    println!("Transpilado com sucesso: {} -> {}", input_path, output_path);
}
