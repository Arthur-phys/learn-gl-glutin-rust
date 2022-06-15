#[cfg(feature = "chapter-1")]
mod _1_getting_started;
#[cfg(feature = "chapter-1")]
use _1_getting_started::*;
mod shader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")] "1_1" => window_color_change(),
        #[cfg(feature = "chapter-1")] "1_2" => draw_triangle(),
        #[cfg(feature = "chapter-1")] "1_3" => draw_rectangle(),
        #[cfg(feature = "chapter-1")] "1_4" => traingles_together(),
        #[cfg(feature = "chapter-1")] "1_5" => different_vao_vbo(),
        #[cfg(feature = "chapter-1")] "1_6" => different_fragment_shaders(),
        #[cfg(feature = "chapter-1")] "1_7" => chaining_variables_in_shaders(),
        #[cfg(feature = "chapter-1")] "1_8" => uniforms_in_shaders(),
        #[cfg(feature = "chapter-1")] "1_9" => fragment_shader_interpolation(),
        #[cfg(feature = "chapter-1")] "1_10" => upside_down(),
        _     => println!("Unknown tutorial id")
    }
}