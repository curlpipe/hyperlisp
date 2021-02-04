// Hyperlisp parser is a markup language for the web inspired by Lisp
#![warn(clippy::all, clippy::pedantic)]

// Import pest, a parsing library
extern crate pest;
#[macro_use]
extern crate pest_derive;

// Import other libraries
use ezcli::{name::Name, named_flag, option};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::{fmt, fs};

// These are html tags that don't require an end tag
const VOID: [&str; 17] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "command", "keygen", "menuitem",
];

// Create a parser from the grammar
#[derive(Parser)]
#[grammar = "hyperlisp.pest"]
pub struct HyperlispParser;

// Create an enum to allow representation as a tree structure
#[derive(Debug)]
pub enum Hyperlisp<'a> {
    Tag(&'a str, HashMap<&'a str, &'a str>, Vec<Hyperlisp<'a>>),
    Text(&'a str),
}

fn main() {
    // Provide command line options
    named_flag!(h, Name::new("help", "h"));
    option!(input);
    option!(output);
    // Show help message if needed
    if h { help(); }
    // Check for input file
    if let Some(input) = input {
        // An input file was provided, read and parse it
        let input = fs::read_to_string(input).unwrap();
        let p = HyperlispParser::parse(Rule::root, &input);
        // Convert into the tree structure and then reformat as html
        let tree: String = root(p.unwrap().next().unwrap())
            .iter()
            .map(|x| format!("{}\n", x))
            .collect();
        // Remove leading and trailing newline characters
        let tree = tree.trim_matches('\n');
        // Check for output file
        if let Some(file) = output {
            // Output file was provided, save to file
            if fs::write(&file, tree).is_ok() {
                println!("File saved successfully to {}", file);
            } else {
                eprintln!("File failed to save to {}", file);
            }
        } else {
            // Output file wasn't provided, print tree
            println!("{}", tree);
        }
    } else {
        // Input file wasn't provided, print help
        help();
    }
}

fn help() -> bool {
    // Function to print help message and exit successfully
    println!(
        "Hyperlisp v{}
Usage:
    hyperlisp [options]
Options:
    --help | -h : Show this help message
    --input [file] | -i [file]: Specify an input file
    --output [file] | -o [file]: Specify an output file
",
        env!("CARGO_PKG_VERSION")
    );
    std::process::exit(0);
}

fn root(p: Pair<Rule>) -> Vec<Hyperlisp> {
    // root = { (WHITESPACE | comment | tag)* }
    let mut tags = vec![];
    for t in p.into_inner() {
        tags.push(match t.as_rule() {
            Rule::comment => continue,
            Rule::tag => tag(t),
            _ => unreachable!(),
        })
    }
    tags
}

fn tag(t: Pair<Rule>) -> Hyperlisp {
    // tag = ${ "(" ~ id ~ (" " ~ attribute)* ~ (" "? ~ body)? ~ ")" }
    let mut t = t.into_inner();
    let id = t.next().unwrap();
    let mut attributes = HashMap::new();
    let mut ats = if let Some(n) = t.next() {
        n
    } else {
        return Hyperlisp::Tag(id.as_str(), attributes, vec![]);
    };
    while let Rule::attribute = ats.as_rule() {
        let mut p = ats.clone().into_inner();
        let name = p.next().unwrap().as_str();
        let val = p.next().unwrap().as_str();
        attributes.insert(name, val);
        if let Some(n) = t.next() {
            ats = n;
        } else {
            break;
        }
    }
    Hyperlisp::Tag(id.as_str(), attributes, body(ats))
}

fn body(b: Pair<Rule>) -> Vec<Hyperlisp> {
    // body = ${ (comment | tag | text)+ }
    let mut result = vec![];
    for i in b.into_inner() {
        result.push(match i.as_rule() {
            Rule::tag => tag(i),
            Rule::text => Hyperlisp::Text(i.as_str()),
            Rule::comment => continue,
            Rule::name => return result,
            _ => unreachable!(),
        })
    }
    result
}

// Implement display trait for hyperlisp to allow html creation
impl<'a> fmt::Display for Hyperlisp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Determine the type of tag
        match self {
            // This is just text, display it
            Self::Text(s) => write!(f, "{}", s),
            // This is a tag, display it
            Self::Tag(i, a, b) => {
                // Read attributes and convert to html
                let mut attributes = vec![];
                for (k, v) in a {
                    attributes.push(format!(" {}=\"{}\"", k, v))
                }
                // Read body and convert to html
                let mut body = vec![];
                for p in b {
                    body.push(format!("{}", p))
                }
                // Format the tag into html
                if VOID.contains(i) && body.is_empty() {
                    // End tag isn't needed, don't include it
                    write!(f, "<{id}{attr}>", id = i, attr = attributes.join(""),)
                } else {
                    // End tag is needed here, include it
                    write!(
                        f,
                        "<{id}{attr}>{body}</{id}>",
                        id = i,
                        attr = attributes.join(""),
                        body = body.join(""),
                    )
                }
            }
        }
    }
}
