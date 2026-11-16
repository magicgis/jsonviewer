#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod common;
mod diff;
mod highlight;
mod i18n;
mod icons;
mod import;
mod json;
mod settings;
mod table;
mod text;
mod tree;
mod update;

fn main() -> eframe::Result {
    // 启动计时日志：%TEMP%\jsonviewer-startup.log
    let _ = std::fs::write(std::env::temp_dir().join("jsonviewer-startup.log"), "startup\n");
    app::startup_log_pub("main begin");

    // 清理旧 exe 不阻塞窗口创建。
    std::thread::spawn(|| update::Updater::cleanup());

    // 与 GL 初始化并行读系统字体。
    let font_rx = app::spawn_system_font_loader();
    app::startup_log_pub("font thread started");

    let files: Vec<std::path::PathBuf> = std::env::args_os().skip(1).map(Into::into).collect();
    app::startup_log_pub("args parsed");

    // 窗口标题栏图标用 32×32（轻量）；exe 图标仍由 build.rs 嵌入 icon.ico。
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_app_id("jsonviewer")
            .with_title("JSON 查看器")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([480.0, 320.0])
            .with_drag_and_drop(true)
            .with_icon(
                eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon-32.png"))
                    .expect("icon-32.png must be valid PNG"),
            ),
        persist_window: true,
        ..Default::default()
    };
    app::startup_log_pub("before run_native");
    eframe::run_native(
        "JSON 查看器",
        options,
        Box::new(move |cc| {
            app::startup_log_pub("creation callback");
            Ok(Box::new(app::App::new(cc, files, font_rx)))
        }),
    )
}
