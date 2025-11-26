use gpui::App;

mod willow_panel;
mod willow_ui;

pub fn init(cx: &mut App) {
    willow_ui::init(cx);
    willow_panel::init(cx);
}
