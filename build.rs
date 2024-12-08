// extern crate embed_resource;
// fn main() {
//     embed_resource::compile("./icon.rc");
// }

use winres;

fn main() {
    let mut res = winres::WindowsResource::new();

    res.set_icon("icon.ico");

    res.compile().unwrap();
}
