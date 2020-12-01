use glutin::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> anyhow::Result<()> {
    unsafe {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("grr - Triangle")
            .with_inner_size(LogicalSize {
                width: 1024.0,
                height: 768.0,
            });
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_srgb(true)
            .with_gl_debug_flag(true)
            .build_windowed(wb, &event_loop)?
            .make_current()
            .unwrap();

        let PhysicalSize {
            width: w,
            height: h,
        } = window.window().inner_size();

        let grr = grr::Device::new(
            |symbol| window.get_proc_address(symbol) as *const _,
            grr::Debug::Enable {
                callback: |report, _, _, _, msg| {
                    println!("{:?}: {:?}", report, msg);
                },
                flags: grr::DebugReport::FULL,
            },
        );

        let spirv = include_bytes!(env!("shader.spv"));

        let vs = grr.create_shader(
            grr::ShaderStage::Vertex,
            grr::ShaderSource::Spirv {
                entrypoint: "main_vs",
            },
            &spirv[..],
            grr::ShaderFlags::VERBOSE,
        )?;
        let fs = grr.create_shader(
            grr::ShaderStage::Fragment,
            grr::ShaderSource::Spirv {
                entrypoint: "main_fs",
            },
            &spirv[..],
            grr::ShaderFlags::VERBOSE,
        )?;

        let vertex_array = grr.create_vertex_array(&[])?;
        let pipeline = grr.create_graphics_pipeline(
            grr::VertexPipelineDesc {
                vertex_shader: vs,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: Some(fs),
            },
            grr::PipelineFlags::VERBOSE,
        )?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::LoopDestroyed => {
                    grr.delete_shaders(&[vs, fs]);
                    grr.delete_pipeline(pipeline);
                    return;
                }
                Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        window.resize(size);
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    grr.bind_pipeline(pipeline);
                    grr.bind_vertex_array(vertex_array);
                    grr.set_viewport(
                        0,
                        &[grr::Viewport {
                            x: 0.0,
                            y: 0.0,
                            w: w as _,
                            h: h as _,
                            n: 0.0,
                            f: 1.0,
                        }],
                    );
                    grr.set_scissor(
                        0,
                        &[grr::Region {
                            x: 0,
                            y: 0,
                            w: w as _,
                            h: h as _,
                        }],
                    );

                    grr.clear_attachment(
                        grr::Framebuffer::DEFAULT,
                        grr::ClearAttachment::ColorFloat(0, [0.5, 0.5, 0.5, 1.0]),
                    );
                    grr.draw(grr::Primitive::Triangles, 0..3, 0..1);

                    window.swap_buffers().unwrap();
                }
                _ => (),
            }
        })
    }
}
