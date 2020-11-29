use std::vec::Vec;
use wgpu;
use winit;

struct Session {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub enum Stage<'a> {
    Radial(RadialStage<'a>),
}

pub struct StageData<'a> {
    pub pipeline: wgpu::RenderPipeline,
    pub input_texture: &'a wgpu::Texture,
    pub render_texture_view: Option<&'a wgpu::TextureView>,
}

pub struct RadialStage<'a> {
    pub stage_data: StageData<'a>,
}

fn create_textures(session: &Session, final_w: u32, final_h: u32) -> Vec<wgpu::Texture> {
    let size = std::cmp::max(final_w, final_h);
    let input_texture: wgpu::Texture;
    let intermediate_texture: wgpu::Texture;
    vec![input_texture, intermediate_texture]
}

fn create_stages<'a>(
    session: &Session,
    textures: &'a Vec<wgpu::Texture>,
    texture_views: &'a Vec<wgpu::TextureView>,
    final_w: u32,
    final_h: u32,
) -> Vec<Stage<'a>> {
    let intermediate_size = std::cmp::max(final_w, final_h);

    let mut stages = Vec::<Stage>::new();

    stages
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).expect("Couldn't create window.");
    wgpu_subscriber::initialize_default_subscriber(None);
    // Temporarily avoid srgb formats for the swapchain on the web
    let device: wgpu::Device;
    let queue: wgpu::Queue;

    let session = Session { device, queue };
    let w = 640;
    let h = 640;

    let textures = create_textures(&session, w, h);
    let texture_views: Vec<wgpu::TextureView>;
    let stages = create_stages(&session, &textures, &texture_views, w, h);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&session, &textures, &texture_views, &stages);
    });
}
