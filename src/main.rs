mod app;
mod kubernetes;
mod settings;
mod ui;

use gpui::*;

actions!(kubespark, [Quit, ToggleSettings]);

fn main() {
    env_logger::init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = runtime.enter();

    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.on_action(quit);
        cx.on_action(toggle_settings);
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-,", ToggleSettings, None),
        ]);

        let bounds = Bounds::centered(None, size(px(1400.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: if cfg!(target_os = "macos") {
                        Some(point(px(8.0), px(8.0)))
                    } else {
                        None
                    },
                }),
                window_min_size: Some(gpui::Size {
                    width: px(800.),
                    height: px(600.),
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| app::KubeSparkApp::new(cx)),
        )
        .unwrap();
    });
}

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn toggle_settings(_: &ToggleSettings, _cx: &mut App) {
    // This action is handled by the KubeSparkApp instance
    // Actions propagate to focused views automatically
}
