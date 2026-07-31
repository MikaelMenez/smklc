/*div(class="",id=""){
|br|

}
*/

use std::fs;

/*
<div class="" id="" ><br></div>



 */
fn get_attrs(text: &[char]) -> (Vec<Atributte>, usize) {
    let mut attrs = vec![];
    let mut ptr: usize = 0;
    let mut i: usize = 0;
    loop {
        if i >= text.len() {
            break;
        }
        while text[i] != '=' {
            i += 1;
        }

        let name: String = text[ptr..i].iter().collect();

        i += 2;
        ptr = i; //olha aq
        while text[i] != '\"' {
            i += 1
        }

        let content: String = text[ptr..i].iter().collect();
        attrs.push(Atributte { name, content });
        i += 1;
        if i < text.len() && text[i] == ',' {
            i += 1;
            ptr = i;
        } else {
            break;
        }
    }
    (attrs, i + 2)
}
#[derive(Debug)]
struct Atributte {
    name: String,
    content: String,
}
#[derive(Debug)]
struct Tag {
    name: String,
    atributtes: Vec<Atributte>,
    content: String,
    tag: Option<Box<Tags>>,
}
#[derive(Debug)]
enum Tags {
    VoidTag(VoidTag),
    Tag(Tag),
}
#[derive(Debug)]
struct VoidTag {
    name: String,
    atributtes: Vec<Atributte>,
}
impl VoidTag {
    fn new(name: String, atributtes: Vec<Atributte>) -> Self {
        VoidTag {
            name: name,
            atributtes: atributtes,
        }
    }
}
fn main() {
    let mut arq: String = fs::read_to_string("test.smkl").unwrap();
    arq = arq.trim().to_string();
    println!("{arq}\n\n\n");

    println!("{:?}", get_attrs(&arq.chars().collect::<Vec<char>>()));
}
