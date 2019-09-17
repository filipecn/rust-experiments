extern crate tobj;
use std::path::Path;

pub fn load_obj(file: &String) {
    let obj = tobj::load_obj(&Path::new(file));
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_mesh() {}
}
