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
        #[cfg(feature = "chapter-1")] "1_11" => offset_triangle(),
        #[cfg(feature = "chapter-1")] "1_12" => position_interpolation(),
        #[cfg(feature = "chapter-1")] "1_13" => load_texture(),
        #[cfg(feature = "chapter-1")] "1_14" => double_texture(),
        #[cfg(feature = "chapter-1")] "1_15" => flip_horizontally_fragment_shader(),
        #[cfg(feature = "chapter-1")] "1_16" => different_wrappers_for_diff_textures(),
        #[cfg(feature = "chapter-1")] "1_17" => vary_visibility(),
        #[cfg(feature = "chapter-1")] "1_18" => vary_visibility_uniform(),
        #[cfg(feature = "chapter-1")] "1_19" => transform_function(),
        #[cfg(feature = "chapter-1")] "1_20" => transform_over_time(),
        #[cfg(feature = "chapter-1")] "1_21" => inverse_transform(),
        #[cfg(feature = "chapter-1")] "1_22" => double_rotating_containers(),
        #[cfg(feature = "chapter-1")] "1_23" => plane_on_x_axis(),
        #[cfg(feature = "chapter-1")] "1_24" => d3_box(),
        #[cfg(feature = "chapter-1")] "1_25" => ten_boxes(),
        #[cfg(feature = "chapter-1")] "1_26" => camera_roam_box(),
        #[cfg(feature = "chapter-1")] "1_27" => free_roam_camera(),
        #[cfg(feature = "chapter-1")] "1_28" => rotating_camera(),
        _     => println!("Unknown tutorial id")
    }
}