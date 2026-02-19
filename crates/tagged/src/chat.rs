/// ChatUi is a `Workspace` item, rendering into the tab window
use zed::unstable::{
    gpui::{EventEmitter, FocusHandle, Focusable},
    ui::{App, Context, IntoElement, Render, SharedString, Window, div},
    workspace::Item,
};

pub struct ChatUi {
    focus_handle: FocusHandle,
}

impl Render for ChatUi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl ChatUi {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for ChatUi {
    fn focus_handle(&self, cx: &App) -> zed::unstable::gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
type ChatEvent = ();
impl EventEmitter<ChatEvent> for ChatUi {}
impl Item for ChatUi {
    type Event = ChatEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Chat".into()
    }
}
