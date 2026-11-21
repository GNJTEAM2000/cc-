use crate::{config::AppConfig, diagnostics::Diagnostics, sandbox};
use eframe::egui::{self, Color32, RichText};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tab { Dashboard, Visuals, Lab, Settings }

pub struct NightfallApp {
    config: AppConfig,
    diagnostics: Diagnostics,
    tab: Tab,
    panel_open: bool,
    status: String,
    last_refresh: std::time::Instant,
}

impl Default for NightfallApp {
    fn default() -> Self {
        Self { config: AppConfig::load(), diagnostics: Diagnostics::new(), tab: Tab::Dashboard, panel_open: true, status: "Ready".into(), last_refresh: std::time::Instant::now() }
    }
}

pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.window_fill = Color32::from_rgb(15, 12, 22);
    style.visuals.panel_fill = Color32::from_rgb(19, 15, 28);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(111, 62, 157);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(75, 48, 103);
    style.spacing.item_spacing = egui::vec2(10.0, 9.0);
    style.spacing.button_padding = egui::vec2(12.0, 7.0);
    ctx.set_style(style);
}

impl NightfallApp {
    fn accent(&self) -> Color32 { Color32::from_rgb(self.config.accent_r, self.config.accent_g, self.config.accent_b) }
    fn nav_button(&mut self, ui: &mut egui::Ui, tab: Tab, label: &str) {
        let selected = self.tab == tab;
        let text = if selected { RichText::new(label).strong().color(self.accent()) } else { RichText::new(label).color(Color32::from_rgb(190, 183, 201)) };
        if ui.selectable_label(selected, text).clicked() { self.tab = tab; }
    }

    fn dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("System overview").color(self.accent()));
        ui.label("Live local diagnostics — no game process access.");
        ui.add_space(12.0);
        egui::Grid::new("stats").num_columns(2).spacing([12.0, 12.0]).show(ui, |ui| {
            stat_card(ui, "CPU USAGE", format!("{:.1}%", self.diagnostics.cpu_percent), self.accent());
            stat_card(ui, "MEMORY USED", format!("{} / {} MB", self.diagnostics.memory_used_mb, self.diagnostics.memory_total_mb), self.accent());
            ui.end_row();
            stat_card(ui, "SANDBOX ENTITIES", "3 demo entities".into(), self.accent());
            stat_card(ui, "CLIENT MODE", "Standalone local".into(), Color32::from_rgb(75, 210, 130));
            ui.end_row();
        });
        ui.add_space(14.0);
        if ui.button("Refresh diagnostics").clicked() { self.diagnostics.refresh(); self.status = "Diagnostics refreshed".into(); }
    }

    fn visuals(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Visual sandbox").color(self.accent()));
        ui.label("These visual controls affect only the built-in demo scene.");
        ui.checkbox(&mut self.config.show_demo_boxes, "Bounding boxes");
        ui.checkbox(&mut self.config.show_demo_names, "Entity labels");
        ui.checkbox(&mut self.config.show_demo_health, "Health indicators");
        ui.checkbox(&mut self.config.show_demo_distance, "Demo distance labels");
        ui.add(egui::Slider::new(&mut self.config.max_demo_distance, 25.0..=500.0).text("Maximum demo distance (m)"));
        ui.separator();
        sandbox::draw_preview(ui, &sandbox::demo_entities(), self.config.show_demo_boxes, self.config.show_demo_names, self.config.show_demo_health, self.config.show_demo_distance, self.config.max_demo_distance);
    }

    fn lab(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Offline feature lab").color(self.accent()));
        ui.label("Training switches run only against synthetic sandbox data.");
        ui.checkbox(&mut self.config.aim_simulation, "Aim-selection simulation");
        ui.checkbox(&mut self.config.trigger_simulation, "Trigger-event simulation");
        ui.checkbox(&mut self.config.vehicle_simulation, "Vehicle-physics simulation");
        ui.add_enabled(self.config.vehicle_simulation, egui::Slider::new(&mut self.config.vehicle_multiplier, 1.0..=3.0).text("Demo speed multiplier"));
        ui.separator();
        if self.config.aim_simulation { ui.label(RichText::new("Aim lab: synthetic target selection enabled").color(Color32::LIGHT_GREEN)); }
        if self.config.trigger_simulation { ui.label(RichText::new("Trigger lab: local event timing enabled").color(Color32::LIGHT_GREEN)); }
        if self.config.vehicle_simulation { ui.label(RichText::new(format!("Vehicle lab: physics multiplier {:.1}x", self.config.vehicle_multiplier)).color(Color32::LIGHT_GREEN)); }
        if !self.config.aim_simulation && !self.config.trigger_simulation && !self.config.vehicle_simulation { ui.label("No lab modules active."); }
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Settings").color(self.accent()));
        ui.label("Accent color");
        let mut rgb = [self.config.accent_r, self.config.accent_g, self.config.accent_b];
        if ui.color_edit_button_srgb(&mut rgb).changed() { self.config.accent_r = rgb[0]; self.config.accent_g = rgb[1]; self.config.accent_b = rgb[2]; }
        if ui.button("Save configuration").clicked() {
            self.status = match self.config.save() { Ok(()) => "Configuration saved".into(), Err(e) => format!("Save failed: {e}") };
        }
        if ui.button("Restore defaults").clicked() { self.config = AppConfig::default(); self.status = "Defaults restored (save to persist)".into(); }
    }
}

impl eframe::App for NightfallApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Insert toggles the panel while this standalone app has keyboard focus.
        if ctx.input(|i| i.key_pressed(egui::Key::Insert)) { self.panel_open = !self.panel_open; }
        if self.last_refresh.elapsed() >= std::time::Duration::from_secs(1) { self.diagnostics.refresh(); self.last_refresh = std::time::Instant::now(); }
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        if !self.panel_open {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(180.0);
                    ui.heading(RichText::new("NIGHTFALL is minimized").color(self.accent()));
                    ui.label("Press INSERT while this window is focused to reopen the panel.");
                });
            });
            return;
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("NIGHTFALL").strong().color(self.accent()));
                ui.label(RichText::new("LOCAL CLIENT  /  0.2.1").small().color(Color32::GRAY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(RichText::new("● LOCAL SANDBOX").color(Color32::from_rgb(90, 215, 145))); });
            });
        });
        egui::SidePanel::left("navigation").resizable(false).default_width(165.0).show(ctx, |ui| {
            ui.add_space(10.0);
            self.nav_button(ui, Tab::Dashboard, "Dashboard");
            self.nav_button(ui, Tab::Visuals, "Visual sandbox");
            self.nav_button(ui, Tab::Lab, "Offline feature lab");
            self.nav_button(ui, Tab::Settings, "Settings");
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| { ui.label(RichText::new(&self.status).small().color(Color32::GRAY)); });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                match self.tab { Tab::Dashboard => self.dashboard(ui), Tab::Visuals => self.visuals(ui), Tab::Lab => self.lab(ui), Tab::Settings => self.settings(ui) }
            });
        });
    }
}

fn stat_card(ui: &mut egui::Ui, title: &str, value: String, accent: Color32) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(300.0);
        ui.label(RichText::new(title).small().color(Color32::GRAY));
        ui.label(RichText::new(value).size(22.0).strong().color(accent));
    });
}
