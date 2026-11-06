#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 860.0])
            .with_min_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Glow,
        multisampling: 0,
        vsync: true,
        ..Default::default()
    };
    eframe::run_native(
        "Rust DS Lab",
        options,
        Box::new(|cc| Ok(Box::new(rust_ds_lab::desktop::Lab::new(cc)))),
    )
}
