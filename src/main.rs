/*div(class="",id=""){
|br|

}
*/

use std::fs;

/*
<div class="" id="" ><br></div>



 */
fn get_tag(txt: &[char]) -> Vec<Tags> {
    let mut ptr1: usize;
    let mut ptr2: usize;
    let mut tags: Vec<Tags> = vec![];
    let mut i = 0;
    let mut atributs: bool = false;
    let mut attrptr: usize;
    while i < txt.len() {
        let mut actual = txt[i];
        if actual == '|' {
            ptr1 = i;
            i += 1;
            actual = txt[i];
            while actual != '|' {
                //mexer aqui para ele reconhecer atributos
                if actual == '(' {
                    atributs = true;
                    attrptr = i;
                    let mut atributtes: Vec<Atributte> = vec![];
                    i += 1;
                    actual = txt[i];
                    let mut tempptr1 = i;
                    let mut tempptr2 = i;
                    while actual != ')' {
                        if actual == '=' {
                            let name = txt[tempptr1..tempptr2].iter().collect::<String>();
                            i += 1;
                            actual = txt[i];
                            let mut tempptr1: usize;
                            let mut tempptr2 = i;
                            if actual == '\"' {
                                i += 1;
                                actual = txt[i];
                                tempptr1 = i;
                                while actual != '\"' {
                                    i += 1;
                                    actual = txt[i];
                                    tempptr2 += 1;
                                }
                                let content =
                                    txt[tempptr1 + 1..tempptr2].iter().collect::<String>();
                                atributtes.push(Atributte { name, content });
                            };
                        }
                        i += 1;
                        actual = txt[i];
                        tempptr2 = i;
                    }
                }
                if actual == '\0' {
                    panic!("non finished void tag");
                }
                i += 1;
                actual = txt[i];
            }

            ptr2 = i;

            tags.push(Tags::VoidTag(VoidTag::new(
                txt[ptr1 + 1..ptr2].iter().collect::<String>(),
                Vec::with_capacity(1),
            )));
        }
        i += 1;
    }
    tags
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
    let mut vec_chars: Vec<char> = arq.chars().collect();
    vec_chars.push('\0');
    println!("{:?}\n\n\n\n\n\n\n", get_tag(&vec_chars));
    println!("{arq}");
}
