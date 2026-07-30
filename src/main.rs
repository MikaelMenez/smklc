/*h1(class="",id=""){


}
*/

use std::fs;

/*
<h1 class="" id="" ></h1>



 */

struct Stag {
    name: String,
    tag: Option<Box<Stag>>,
}
enum Tag {
    Class(String),
    Id(String),
}
fn main() {
    let mut arq: String = fs::read_to_string("test.smkl").unwrap();
    arq = arq.trim().to_string();
    let vec_chars: Vec<char> = arq.chars().collect();
}
