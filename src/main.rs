use glutin::dpi::LogicalSize;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_title("grr - Triangle")
            .with_dimensions(LogicalSize {
                width: 1024.0,
                height: 768.0,
            });
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_srgb(true)
            .with_gl_debug_flag(true)
            .build_windowed(wb, &events_loop)?
            .make_current()
            .unwrap();

        let LogicalSize {
            width: w,
            height: h,
        } = window.window().get_inner_size().unwrap();

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
            grr::ShaderSource::Spirv { entrypoint: "main_vs" },
            &spirv[..],
            grr::ShaderFlags::VERBOSE,
        )?;
        let fs = grr.create_shader(
            grr::ShaderStage::Fragment,
            grr::ShaderSource::Spirv { entrypoint: "main_fs" },
            &spirv[..],
            grr::ShaderFlags::VERBOSE,
        )?;

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

        let mut running = true;
        while running {
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(size) => {
                        let dpi_factor = window.window().get_hidpi_factor();
                        window.resize(size.to_physical(dpi_factor));
                    }
                    _ => (),
                },
                _ => (),
            });
            grr.bind_pipeline(pipeline);
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

            window.swap_buffers()?;
        }

        grr.delete_shaders(&[vs, fs]);
        grr.delete_pipeline(pipeline);
    }

    Ok(())
}
