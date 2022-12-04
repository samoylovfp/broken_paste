use eframe::egui::TextEdit;

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "egui",
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(MyEguiApp::new(cc))),
        )
        .await
        .unwrap();
    });
}

#[derive(Default)]
struct MyEguiApp {
    text: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default()
            .show(ctx, |ui| TextEdit::multiline(&mut self.text).show(ui));
    }
}
