/*h1(class="",id=""){


}
*/

use std::fs;

/*
<h1 class="" id="" ></h1>



 */
struct Atributte {
    name: String,
    content: String,
}
struct Tag {
    name: String,
    atributte: Atributte,
    content: String,
    tag: Option<Box<Tags>>,
}
enum Tags {
    Stag(String),
    Tag(Tag),
}
fn main() {
    let mut arq: String = fs::read_to_string("test.smkl").unwrap();
    arq = arq.trim().to_string();
    let vec_chars: Vec<char> = arq.chars().collect();
}
