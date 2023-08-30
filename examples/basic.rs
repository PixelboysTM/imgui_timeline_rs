use std::time::Instant;

use imgui::{ConfigFlags, Direction, FontSource, StyleColor};
use imgui_wgpu::{Renderer, RendererConfig};
use pollster::block_on;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

const PRESENT_MODE: wgpu::PresentMode = wgpu::PresentMode::AutoNoVsync;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let (window, size, surface) = {
        let version = env!("CARGO_PKG_VERSION");

        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 800.0,
            height: 600.0,
        });
        window.set_title(&format!("imgui-timeline-rs {version}"));
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        (window, size, surface)
    };

    let hidpi_factor = window.scale_factor();

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) =
        block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

    let surface_desc = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: PRESENT_MODE,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
    };

    surface.configure(&device, &surface_desc);

    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (18.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    // imgui.fonts().add_font(&[FontSource::DefaultFontData {
    //     config: Some(imgui::FontConfig {
    //         oversample_h: 1,
    //         pixel_snap_h: true,
    //         size_pixels: font_size,
    //         ..Default::default()
    //     }),
    // }]);
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("./RobotoMono-Regular.ttf"),
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            size_pixels: font_size,
            oversample_h: 1,
            pixel_snap_h: true,
            ..Default::default()
        }),
    }]);

    //
    // Set up dear imgui wgpu renderer
    //
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    let renderer_config = RendererConfig {
        texture_format: surface_desc.format,
        ..Default::default()
    };

    let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    set_dark_theme(&mut imgui);

    imgui.io_mut().config_flags |= ConfigFlags::DOCKING_ENABLE | ConfigFlags::VIEWPORTS_ENABLE;

    let mut last_frame = Instant::now();
    let mut demo_open = true;

    let mut last_cursor = None;

    let mut timeline = imgui_timeline_rs::Timeline::new("Basic");

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let surface_desc = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    width: size.width,
                    height: size.height,
                    present_mode: PRESENT_MODE,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
                };

                surface.configure(&device, &surface_desc);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let delta_s = last_frame.elapsed();
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {e:?}");
                        return;
                    }
                };
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                {
                    ui.dockspace_over_main_viewport();
                    ui.show_demo_window(&mut demo_open);
                    imgui_timeline_rs::hello(ui);
                    timeline.draw(ui, delta_s.as_secs_f32());

                    // let window = ui.window("Hello world");
                    // window
                    //     .size([300.0, 100.0], Condition::FirstUseEver)
                    //     .build(|| {
                    //         ui.text("Hello world!");
                    //         ui.text("This...is...imgui-rs on WGPU!");
                    //         ui.separator();
                    //         let mouse_pos = ui.io().mouse_pos;
                    //         ui.text(format!(
                    //             "Mouse Position: ({:.1},{:.1})",
                    //             mouse_pos[0], mouse_pos[1]
                    //         ));
                    //     });

                    // let window = ui.window("Hello too");
                    // window
                    //     .size([400.0, 200.0], Condition::FirstUseEver)
                    //     .position([400.0, 200.0], Condition::FirstUseEver)
                    //     .build(|| {
                    //         ui.text(format!("Frametime: {delta_s:?}"));
                    //     });
                }

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if last_cursor != Some(ui.mouse_cursor()) {
                    last_cursor = Some(ui.mouse_cursor());
                    platform.prepare_render(ui, &window);
                }

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                renderer
                    .render(imgui.render(), &queue, &device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass);

                queue.submit(Some(encoder.finish()));

                frame.present();
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

fn set_dark_theme(context: &mut imgui::Context) {
    let styles = context.style_mut();

    styles.window_padding = [8.0, 8.0];
    styles.frame_padding = [5.0, 2.0];
    styles.cell_padding = [6.0, 6.0];
    styles.item_spacing = [6.0, 6.0];
    styles.item_inner_spacing = [6.0, 6.0];
    styles.touch_extra_padding = [0.0, 0.0];
    styles.indent_spacing = 25.0;
    styles.scrollbar_size = 15.0;
    styles.grab_min_size = 10.0;
    styles.window_border_size = 1.0;
    styles.child_border_size = 1.0;
    styles.popup_border_size = 1.0;
    styles.frame_border_size = 1.0;
    styles.tab_border_size = 1.0;
    styles.window_rounding = 7.0;
    styles.child_rounding = 4.0;
    styles.frame_rounding = 3.0;
    styles.popup_rounding = 4.0;
    styles.scrollbar_rounding = 9.0;
    styles.grab_rounding = 3.0;
    styles.log_slider_deadzone = 4.0;
    styles.tab_rounding = 4.0;

    styles.window_title_align = [0.5, 0.5];
    styles.window_menu_button_position = Direction::None;
    styles.color_button_position = Direction::Left;
    styles.button_text_align = [0.5, 0.5];
    styles.circle_tesselation_max_error = 0.1;

    styles.colors[StyleColor::Text as usize] = [1.00, 1.00, 1.00, 1.00];
    styles.colors[StyleColor::TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    styles.colors[StyleColor::WindowBg as usize] = [0.10, 0.10, 0.10, 1.00];
    styles.colors[StyleColor::ChildBg as usize] = [0.00, 0.00, 0.00, 0.00];
    styles.colors[StyleColor::PopupBg as usize] = [0.19, 0.19, 0.19, 1.00];
    styles.colors[StyleColor::Border as usize] = [0.19, 0.19, 0.19, 0.29];
    styles.colors[StyleColor::BorderShadow as usize] = [0.00, 0.00, 0.00, 0.24];
    styles.colors[StyleColor::FrameBg as usize] = [0.05, 0.05, 0.05, 0.54];
    styles.colors[StyleColor::FrameBgHovered as usize] = [0.19, 0.19, 0.19, 0.54];
    styles.colors[StyleColor::FrameBgActive as usize] = [0.20, 0.22, 0.23, 1.00];
    styles.colors[StyleColor::TitleBg as usize] = [0.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::TitleBgActive as usize] = [0.06, 0.06, 0.06, 1.00];
    styles.colors[StyleColor::TitleBgCollapsed as usize] = [0.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.00];
    styles.colors[StyleColor::ScrollbarBg as usize] = [0.05, 0.05, 0.05, 0.54];
    styles.colors[StyleColor::ScrollbarGrab as usize] = [0.34, 0.34, 0.34, 0.54];
    styles.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.40, 0.40, 0.40, 0.54];
    styles.colors[StyleColor::ScrollbarGrabActive as usize] = [0.56, 0.56, 0.56, 0.54];
    styles.colors[StyleColor::CheckMark as usize] = [0.86, 0.554, 0.33, 1.00];
    styles.colors[StyleColor::SliderGrab as usize] = [0.34, 0.34, 0.34, 0.54];
    styles.colors[StyleColor::SliderGrabActive as usize] = [0.56, 0.56, 0.56, 0.54];
    styles.colors[StyleColor::Button as usize] = [0.05, 0.05, 0.05, 0.54];
    styles.colors[StyleColor::ButtonHovered as usize] = [0.19, 0.19, 0.19, 0.54];
    styles.colors[StyleColor::ButtonActive as usize] = [0.20, 0.22, 0.23, 1.00];
    styles.colors[StyleColor::Header as usize] = [0.00, 0.00, 0.00, 0.52];
    styles.colors[StyleColor::HeaderHovered as usize] = [0.00, 0.00, 0.00, 0.36];
    styles.colors[StyleColor::HeaderActive as usize] = [0.20, 0.22, 0.23, 0.33];
    styles.colors[StyleColor::Separator as usize] = [0.28, 0.28, 0.28, 0.29];
    styles.colors[StyleColor::SeparatorHovered as usize] = [0.44, 0.44, 0.44, 0.29];
    styles.colors[StyleColor::SeparatorActive as usize] = [0.40, 0.44, 0.47, 1.00];
    styles.colors[StyleColor::ResizeGrip as usize] = [0.28, 0.28, 0.28, 0.29];
    styles.colors[StyleColor::ResizeGripHovered as usize] = [0.44, 0.44, 0.44, 0.29];
    styles.colors[StyleColor::ResizeGripActive as usize] = [0.40, 0.44, 0.47, 1.00];
    styles.colors[StyleColor::Tab as usize] = [0.00, 0.00, 0.00, 0.52];
    styles.colors[StyleColor::TabHovered as usize] = [0.14, 0.14, 0.14, 1.00];
    styles.colors[StyleColor::TabActive as usize] = [0.20, 0.20, 0.20, 0.36];
    styles.colors[StyleColor::TabUnfocused as usize] = [0.00, 0.00, 0.00, 0.52];
    styles.colors[StyleColor::TabUnfocusedActive as usize] = [0.14, 0.14, 0.14, 1.00];
    styles.colors[StyleColor::DockingPreview as usize] = [0.86, 0.554, 0.33, 1.00];
    styles.colors[StyleColor::NavHighlight as usize] = [0.86, 0.554, 0.33, 1.00];
    styles.colors[StyleColor::NavWindowingDimBg as usize] = [0.86, 0.554, 0.33, 1.00];
    styles.colors[StyleColor::NavWindowingHighlight as usize] = [0.86, 0.554, 0.33, 1.00];
    // styles.colors[StyleColor::DockingEmptyBg as usize]        = [1.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::PlotLines as usize] = [1.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::PlotHistogram as usize] = [1.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.00, 0.00, 1.00];
    styles.colors[StyleColor::TableHeaderBg as usize] = [0.00, 0.00, 0.00, 0.52];
    styles.colors[StyleColor::TableBorderStrong as usize] = [0.00, 0.00, 0.00, 0.52];
    styles.colors[StyleColor::TableBorderLight as usize] = [0.28, 0.28, 0.28, 0.29];
    styles.colors[StyleColor::TableRowBg as usize] = [0.00, 0.00, 0.00, 0.00];
    styles.colors[StyleColor::TableRowBgAlt as usize] = [1.00, 1.00, 1.00, 0.06];
    styles.colors[StyleColor::TextSelectedBg as usize] = [0.20, 0.22, 0.23, 1.00];
    styles.colors[StyleColor::DragDropTarget as usize] = [0.33, 0.67, 0.86, 1.00];
    // styles.colors[StyleColor::NavHighlight as usize] = [1.00, 0.00, 0.00, 1.00];
    // styles.colors[StyleColor::NavWindowingHighlight as usize] = [1.00, 0.00, 0.00, 0.70];
    // styles.colors[StyleColor::NavWindowingDimBg as usize] = [1.00, 0.00, 0.00, 0.20];
    styles.colors[StyleColor::ModalWindowDimBg as usize] = [1.00, 0.00, 0.00, 0.35];
}
