/*div(class="",id=""){
|br|

}
*/

use std::fs;

/*
<div class="" id="" ><br></div>



 */
fn get_attrs(text: &[char]) -> Vec<Atributte> {
    let mut attrs = vec![];
    let mut ptr: usize = 0;
    let mut offset: usize = 0;
    let mut i: usize = 0;
    while text[i] != '=' {
        i += 1;
    }
    offset = i - offset;
    let name: String = text[ptr..ptr + offset].iter().collect();

    offset = 0;
    i += 2;
    ptr = i; //olha aq
    while text[i] != '\"' {
        println!("{}", text[i]);
        i += 1
    }
    offset = i - offset;
    println!("{ptr} {offset}");
    let content: String = text[ptr..offset].iter().collect();
    attrs.push(Atributte { name, content });
    attrs
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
