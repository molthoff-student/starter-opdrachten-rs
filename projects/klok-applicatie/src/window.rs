use glium::glutin::surface::WindowSurface;
use glium::winit::dpi::LogicalSize;
use glium::{Display, Surface};

use imgui::{Context, Ui};

use imgui_glium_renderer::Renderer;

use imgui_winit_support::winit::event::{Event, WindowEvent};
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::WindowAttributes;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

use crate::app::AppData;

pub fn create_window<FInit, FUi>(title: &str, mut startup: FInit, mut run_ui: FUi)
where
    FInit: FnMut(&mut Context, &mut Renderer, &Display<WindowSurface>) + 'static,
    FUi: FnMut(&mut bool, &mut Ui, &mut AppData) + 'static,
{
    let mut app_data = AppData::init();

    let window_size = LogicalSize::new(
        app_data.settings.window_width,
        app_data.settings.window_height,
    );

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_attributes = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(window_size);

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);

    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut platform = WinitPlatform::new(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

    let mut last_frame = Instant::now();

    startup(&mut imgui, &mut renderer, &display);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, ui, &mut app_data);
                if !run {
                    window_target.exit();
                }

                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(ui, &window);
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                    let settings = &mut app_data.settings;
                    settings.window_width = new_size.width;
                    settings.window_height = new_size.height;
                }
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let settings = &mut app_data.settings;
                if settings.exitsave {
                    settings.save().expect("Couldn't save settings.");
                    app_data.alarm_clock.save().expect("Couldn't save alarms.");
                }
                window_target.exit()
            }
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
        })
        .expect("EventLoop error");
}
