use std::path::PathBuf;

use zed::unstable::{
    component::{self, ComponentScope},
    gpui::{AppContext as _, Entity, img},
    ui::{
        ActiveTheme, AnyElement, App, Component, Context, IntoElement, ParentElement as _,
        RegisterComponent, Render, SharedString, Styled, Window, div, h_flex, px, v_flex,
    },
};

pub struct MyState {
    name: SharedString,
}

impl MyState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self { name: "Bob".into() }
    }
}

pub fn init(cx: &mut App) {
    // Create a new Entity using `cx.new`
    let my_state_entity: Entity<MyState> = cx.new(|cx| MyState::new(cx));

    // Use the handle to look up the state from the App cx
    let name: SharedString = my_state_entity.read(cx).name.clone();

    // Use entity.update and provide a closure to edit the instance state
    my_state_entity.update(cx, |it: &mut MyState, cx| {
        it.name = "Carol".into();
    });
}

#[derive(RegisterComponent)]
pub struct ChatBubble {
    //
    display_name: SharedString,
    message: SharedString,
}

impl ChatBubble {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            display_name: "Alice".into(),
            message: "Hey, are you online?".into(),
        }
    }
}

impl Render for ChatBubble {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            //
            .p_2()
            .child(
                //
                h_flex()
                    .flex_shrink()
                    //
                    .bg(cx.theme().colors().panel_background)
                    .p_4()
                    .gap_4()
                    .rounded_bl_lg()
                    .rounded_br_lg()
                    .rounded_tr_lg()
                    .child(
                        //
                        img(PathBuf::from(".assets/tagged.svg"))
                            //
                            .w(px(48.))
                            .rounded_lg(),
                    )
                    .child(
                        v_flex()
                            //
                            .child(
                                //
                                div()
                                    //
                                    .text_lg()
                                    .child(self.display_name.clone()),
                            )
                            .child(
                                //
                                div()
                                    //
                                    .child(self.message.clone()),
                            ),
                    ),
            )
    }
}

impl Component for ChatBubble {
    //
    fn scope() -> ComponentScope {
        ComponentScope::None
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        Some(cx.new(|cx| ChatBubble::new(cx)).into_any_element())
    }
}
