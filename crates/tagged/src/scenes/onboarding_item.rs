use zed::unstable::{
    gpui::{EventEmitter, FocusHandle, Focusable},
    ui::{App, Context, IntoElement, ParentElement, Render, SharedString, Styled, Window, div},
    workspace::Item,
};

pub fn init(_cx: &mut App) {
    // cx.observe_new(|workspace: &mut Workspace, window, cx| {
    //     let Some(window) = window else {
    //         return;
    //     };
    // })
    // .detach();
}

pub struct OnboardingItem {
    focus_handle: FocusHandle,
}

impl OnboardingItem {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            //
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for OnboardingItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            //
            .p_2()
            .child(
                //
                div()
                    //
                    .debug()
                    .child("OnboardingItem"),
            )
    }
}

impl EventEmitter<()> for OnboardingItem {}
impl Focusable for OnboardingItem {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for OnboardingItem {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }
}
