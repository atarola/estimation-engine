use std::iter;

use mustache::Data;
use rand::Rng;

use crate::StaticFiles;

pub fn render_template(name: &str, data: &Data) -> String {
    let index = StaticFiles::get(name).unwrap();
    let template_string = std::str::from_utf8(index.data.as_ref()).unwrap();
    let template = mustache::compile_str(template_string).unwrap();
    template.render_data_to_string(data).unwrap()
}

pub fn generate_id(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
