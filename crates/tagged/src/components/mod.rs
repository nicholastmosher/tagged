use zed::unstable::ui::App;

pub mod profile_bar;
pub mod space_icon;
pub mod tagged_panel;

pub fn init(cx: &mut App) {
    profile_bar::init(cx);
    tagged_panel::init(cx);

    // cx.observe_new(|workspace: &mut Workspace, window, cx| {
    //     let Some(window) = window else {
    //         return;
    //     };
    // })
    // .detach();
}
