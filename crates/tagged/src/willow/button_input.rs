use std::rc::Rc;

use tracing::{debug, info, warn};
use zed::unstable::{
    component,
    editor::Editor,
    gpui::{self, AppContext as _, Entity, Stateful},
    ui::{
        ActiveTheme as _, App, Component, Context, Div, ElementId, FluentBuilder as _, IconButton,
        IconName, IconSize, InteractiveElement as _, IntoElement, ParentElement as _,
        RegisterComponent, Rems, Render, SharedString, StatefulInteractiveElement as _,
        Styled as _, Window, div,
    },
};

#[derive(RegisterComponent)]
pub struct ButtonInput {
    id: ElementId,
    name: SharedString,
    placeholder: Option<SharedString>,
    editor: Option<Entity<Editor>>,
    // on_submit: Option<Rc<dyn Fn(&mut Self, String, &mut Window, &mut Context<Self>)>>,
    on_submit: Vec<Rc<dyn Fn(&mut Self, String, &mut Window, &mut Context<Self>)>>,
}

impl ButtonInput {
    pub fn new(id: impl Into<ElementId>, name: SharedString, _cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            name,
            placeholder: None,
            editor: None,
            on_submit: Default::default(),
        }
    }

    pub fn placeholder_text(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Add a new submit handler for this button.
    ///
    /// All added handlers will be called when the button is pressed.
    pub fn on_submit(
        mut self,
        on_submit: impl Fn(&mut Self, String, &mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        // self.on_submit = Some(Rc::new(move |this, text, window, cx| {
        //     //
        //     on_submit(this, text, window, cx)
        // }));
        self.on_submit.push(Rc::new(move |this, text, window, cx| {
            // Wrap all handlers with these prechecks:
            let button = &this.name;
            if text.is_empty() {
                warn!(%button, "Empty text, noop");
                return;
            }

            (on_submit)(this, text, window, cx)
        }));
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.editor = None;
        self
    }

    /// Opposite state to `is_button`
    pub fn is_input(&self) -> bool {
        self.editor.is_some()
    }

    /// Opposite state to `is_input`
    pub fn is_button(&self) -> bool {
        self.editor.is_none()
    }

    pub fn fresh_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor = Some(cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if let Some(placeholder) = &self.placeholder {
                editor.set_placeholder_text(&**placeholder, window, cx);
            }
            editor
        }));
    }
}

impl Render for ButtonInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let button_side = self.render_button_side(window, cx);
        let input_side = self.render_input_side(window, cx);
        div()
            //
            .id(self.id.clone())
            .text_center()
            .justify_center()
            .border_2()
            .border_dashed()
            .border_color(cx.theme().colors().border.opacity(0.6))
            .rounded_sm()
            // button side
            .when_none(&self.editor, |this| {
                //
                this.child(button_side)
            })
            // input side
            .when_some(self.editor.as_ref(), |this, _editor| {
                //
                this.child(input_side)
            })
    }
}

impl ButtonInput {
    fn render_button_side(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + 'static {
        div()
            //
            .id("todo")
            .px_2()
            .py_4()
            .active(|style| style.bg(cx.theme().colors().ghost_element_active))
            .hover(|style| {
                style
                    .bg(cx.theme().colors().ghost_element_hover)
                    .border_color(cx.theme().colors().border.opacity(1.0))
            })
            .child(
                div()
                    //
                    .text_color(cx.theme().colors().text_muted)
                    .child(
                        //
                        self.name.clone(),
                    ),
            )
            .on_click(cx.listener(|this, _event, window, cx| {
                //
                this.editor = Some(
                    //
                    cx.new(|cx| {
                        let mut editor = Editor::single_line(window, cx);
                        if let Some(placeholder) = &this.placeholder {
                            editor.set_placeholder_text(&**placeholder, window, cx);
                        }
                        editor
                    }),
                );
                cx.notify();
            }))
    }

    fn render_input_side(
        &mut self,
        window: &mut Window,
        cx: &mut Context<'_, ButtonInput>,
    ) -> impl IntoElement + 'static {
        let Some(editor) = self.editor.clone() else {
            return div().p_4().debug();
        };

        let cancel_button = render_icon_button("cancel", IconName::XCircle, window, cx);
        let accept_button = render_icon_button("accept", IconName::ChevronRight, window, cx);

        div()
            //
            .h_full()
            .w_full()
            .flex()
            .flex_row()
            .child(
                cancel_button
                    //
                    .on_click(cx.listener(|this, _event, _window, _cx| {
                        this.editor.take();
                    })),
            )
            .child(
                div()
                    //
                    .px_2()
                    .py_4()
                    .flex_grow()
                    .child(editor.clone()),
            )
            .child(
                accept_button
                    //
                    .on_click(cx.listener({
                        let editor = editor.clone();
                        move |this, _event, window, cx| {
                            let text = editor.read(cx).text(cx);
                            if text.is_empty() {
                                warn!("Empty input, noop");
                                return;
                            };
                            debug!(%text, "ButtonInput click");
                            // if let Some(on_submit) = this.on_submit.clone() {
                            //     info!(%text, "ButtonInput submit");
                            //     (on_submit)(this, text, window, cx)
                            // }
                            for on_submit in this.on_submit.clone() {
                                //
                                info!(%text, "ButtonInput submit");
                                (on_submit)(this, text.clone(), window, cx)
                            }
                        }
                    })),
            )
    }
}

pub fn render_icon_button<T>(
    id: impl std::fmt::Display,
    icon: IconName,
    _window: &mut Window,
    cx: &mut Context<T>,
) -> Stateful<Div> {
    div()
        //
        // Id namespaced by the component id, followed by the passed `id` as a suffix
        .id(SharedString::from(format!("{id}/icon-button")))
        .p_4()
        .active(|style| style.bg(cx.theme().colors().ghost_element_active))
        .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
        .child(
            IconButton::new(SharedString::from(format!("{id}/icon")), icon)
                .icon_size(IconSize::Custom(Rems(1.5))),
        )
}

impl Component for ButtonInput {
    fn preview(_window: &mut Window, cx: &mut App) -> Option<gpui::AnyElement> {
        let ui = cx.new(|cx| {
            Self::new("sample", "Sample".into(), cx)
                .placeholder_text("Input here")
                .on_submit(|_this, _text, _window, _cx| {
                    info!("Sample OnClick");
                })
        });
        let container = div().p_2().child(ui);
        Some(container.into_any_element())
    }
}
