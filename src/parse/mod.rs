use std::str::Chars;

use crate::error::Error;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, text: String) -> Result<String, Error> {
        Ok(Node::parse(&text)?.to_string())
    }
}

#[derive(Debug)]
pub enum Node {
    Empty,
    LineBreak,
    Text(String),

    List(Vec<Node>),
    Paragraph(Box<Node>),
}

impl Node {
    pub fn parse(text: &str) -> Result<Node, Error> {
        let mut nodes = Vec::new();
        let mut buf = String::new();

        let mut first_line_break = false;
        let mut second_line_break = false;
        for line in text.lines() {
            if second_line_break {
                nodes.push(Node::LineBreak);
            }

            if line.trim().is_empty() {
                if first_line_break {
                    second_line_break = true;
                } else {
                    first_line_break = true;
                    nodes.push(Node::Paragraph(Box::new(Node::parse_paragraph(
                        &mut buf.chars(),
                    )?)));
                    buf.clear();
                }
            } else {
                first_line_break = false;
                second_line_break = false;
                buf += line;
            }
        }

        if !buf.is_empty() {
            nodes.push(Node::Paragraph(Box::new(Node::parse_paragraph(
                &mut buf.chars(),
            )?)));
        }

        Ok(match nodes.len() {
            0 => Node::Empty,
            1 => nodes.pop().unwrap(),
            _ => Node::List(nodes),
        })
    }

    fn parse_paragraph(text: &mut Chars<'_>) -> Result<Node, Error> {
        /*let mut nodes = Vec::new();

        let start = 0;
        let mut cur = 0;

        while let Some(ch) = text.next() {
            if ch == '[' {
                nodes.push(Node::Text(text.take(cur).collect()));
                let node =
            } else {
                cur += 1;
            }
        }*/

        Ok(Node::Text(text.collect()))
    }

    /*    fn parse_link(text: &mut Chars<'_>) -> Result<Node, Error> {

    }*/
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Empty => String::new(),
            Node::LineBreak => "\n <br />".to_string(),
            Node::Text(text) => text.clone(),
            Node::List(vec) => {
                let mut output = String::new();
                for node in vec {
                    output += &node.to_string();
                }

                output
            }
            Node::Paragraph(node) => format!("<p>{}</p>", node.to_string()),
        }
    }
}
