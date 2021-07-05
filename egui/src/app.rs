use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui|{
                    ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                    ui.label("e");
                    ui.add(egui::Label::new("iπ").small_raised());
                    ui.label(" + 1 = 0");
                });
                ui.horizontal(|ui|{
                    if ui.button("☀").clicked() {
                        set_theme(false, ctx);
                    };
                    if ui.button("🌑").clicked() {
                        set_theme(true, ctx);
                    };
                })
            });
        });
    }
}

fn set_theme(dark: bool, context: &egui::Context) {
    if dark {
        let visuals = egui::Visuals::dark();
        context.set_visuals(visuals);
    } else {
        let visuals = egui::Visuals::light();
        context.set_visuals(visuals);
    }
}
