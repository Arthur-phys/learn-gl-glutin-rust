use std::fs::{self,File};
use std::env;
use gl;

pub struct Shader {
    pub id: i32,
}

impl Shader {
    pub fn new(vertex_path: &str, shader_path: &str) {
        let vertex_shader = File::open(vertex_path);
        let vertex_shader = File::open(shader_path);
    }
}