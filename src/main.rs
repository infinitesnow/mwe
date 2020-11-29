use std::vec::Vec;
pub enum Stage<'a> {
    Radial(RadialStage<'a>),
}

pub struct StageData<'a> {
    pub input_texture: &'a wgpu::Texture,
}

pub struct RadialStage<'a> {
    pub stage_data: StageData<'a>,
}

fn create_textures() -> Vec<wgpu::Texture> {
    let input_texture: wgpu::Texture;
    let intermediate_texture: wgpu::Texture;
    vec![input_texture, intermediate_texture]
}

fn create_stages<'a>(
    textures: &'a Vec<wgpu::Texture>,
    texture_views: &'a Vec<wgpu::TextureView>,
) -> Vec<Stage<'a>> {
    let mut stages = Vec::<Stage>::new();
    stages
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let textures = create_textures();
    let texture_views: Vec<wgpu::TextureView>;
    let stages = create_stages(&textures, &texture_views);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&textures, &texture_views, &stages);
    });
}
