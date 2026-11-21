use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

#[derive(Clone)]
pub struct DemoEntity {
    pub name: &'static str,
    pub distance_m: f32,
    pub health: f32,
    pub center: Pos2,
    pub size: Vec2,
}

pub fn demo_entities() -> Vec<DemoEntity> {
    vec![
        DemoEntity { name: "Sandbox Alpha", distance_m: 42.0, health: 0.92, center: Pos2::new(0.30, 0.42), size: Vec2::new(62.0, 116.0) },
        DemoEntity { name: "Sandbox Bravo", distance_m: 86.0, health: 0.63, center: Pos2::new(0.57, 0.48), size: Vec2::new(58.0, 108.0) },
        DemoEntity { name: "Sandbox Charlie", distance_m: 134.0, health: 0.28, center: Pos2::new(0.78, 0.38), size: Vec2::new(54.0, 102.0) },
    ]
}

pub fn draw_preview(ui: &mut egui::Ui, entities: &[DemoEntity], show_boxes: bool, show_names: bool, show_health: bool, show_distance: bool, max_distance: f32) {
    let desired = Vec2::new(ui.available_width().max(300.0), 390.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8.0, Color32::from_rgb(13, 11, 20));
    painter.rect_stroke(rect, 8.0, Stroke::new(1.0, Color32::from_rgb(70, 48, 92)));
    painter.text(rect.left_top() + Vec2::new(14.0, 12.0), egui::Align2::LEFT_TOP, "LOCAL SANDBOX PREVIEW", egui::FontId::proportional(12.0), Color32::from_rgb(177, 145, 205));

    for entity in entities.iter().filter(|e| e.distance_m <= max_distance) {
        let center = Pos2::new(rect.left() + rect.width() * entity.center.x, rect.top() + rect.height() * entity.center.y);
        let box_rect = Rect::from_center_size(center, entity.size);
        if show_boxes {
            painter.rect_stroke(box_rect, 2.0, Stroke::new(1.7, Color32::from_rgb(177, 105, 240)));
        }
        if show_names {
            painter.text(Pos2::new(box_rect.center().x, box_rect.top() - 18.0), egui::Align2::CENTER_CENTER, entity.name, egui::FontId::proportional(11.0), Color32::WHITE);
        }
        if show_health {
            let bar = Rect::from_min_size(Pos2::new(box_rect.left() - 8.0, box_rect.top()), Vec2::new(4.0, box_rect.height()));
            painter.rect_filled(bar, 1.0, Color32::from_gray(55));
            painter.rect_filled(Rect::from_min_size(Pos2::new(bar.left(), bar.bottom() - bar.height() * entity.health), Vec2::new(bar.width(), bar.height() * entity.health)), 1.0, Color32::from_rgb(75, 210, 130));
        }
        if show_distance {
            painter.text(Pos2::new(box_rect.center().x, box_rect.bottom() + 12.0), egui::Align2::CENTER_CENTER, format!("{:.0} m", entity.distance_m), egui::FontId::proportional(10.0), Color32::LIGHT_GRAY);
        }
    }
}
