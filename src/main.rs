/*h1(class="",id=""){


}
*/

use std::fs;

/*
<h1 class="" id="" ></h1>



 */
fn get_tag(txt: &[char]) -> Vec<Tag> {
    let mut ptr1: usize = 0;
    let mut ptr2: usize = 0;
    let mut tags: Vec<Tag> = vec![];
    let mut i = 0;
    while i < txt.len() {
        let mut actual = txt[i];
        if actual == '|' {
            ptr1 = i;
            i += 1;
            actual = txt[i];
            while actual != '|' {
                //mexer aqui para ele reconhecer atributos
                i += 1;
                actual = txt[i];
            }
            ptr2 = i;
            tags.push(Tag {
                name: txt[ptr1..=ptr2].iter().collect::<String>(),
                atributtes: Vec::with_capacity(1),
                content: String::new(),
                tag: None,
            });
            ptr1 = i;
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
    tag: Option<Box<Tag>>,
}

fn main() {
    let mut arq: String = fs::read_to_string("test.smkl").unwrap();
    arq = arq.trim().to_string();
    let vec_chars: Vec<char> = arq.chars().collect();
    print!("{:?}", get_tag(&vec_chars));
}
