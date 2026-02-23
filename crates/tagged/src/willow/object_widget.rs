use std::collections::HashMap;

use serde_json::{Map, Value, json};
use zed::unstable::{
    gpui::{AppContext as _, Entity},
    ui::{
        ActiveTheme, App, Context, InteractiveElement, IntoElement, ParentElement as _, Render,
        SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    },
};

fn init(cx: &mut App) {
    // let _widget = cx.new(|cx| ObjectWidget::new(json!({}), cx));
}

pub struct ObjectWidget {
    //
    value: Entity<Value>,

    current_path: JsonPath,
    by_path: HashMap<JsonPath, PerPathState>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct JsonPath {
    /// Default value [] acts as root path
    path: Vec<JsonIndex>,
}

impl JsonPath {
    pub fn push_index(&mut self, index: impl Into<JsonIndex>) -> &mut Self {
        self.path.push(index.into());
        self
    }

    pub fn pop(&mut self) -> Option<JsonIndex> {
        self.path.pop()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsonIndex {
    Key(String),
    Number(usize),
}

impl From<usize> for JsonIndex {
    fn from(value: usize) -> Self {
        Self::Number(value)
    }
}

impl From<String> for JsonIndex {
    fn from(value: String) -> Self {
        Self::Key(value)
    }
}

#[derive(Debug)]
pub struct PerPathState {
    //
    open: bool,
}

impl ObjectWidget {
    pub fn new(value: Value, cx: &mut Context<Self>) -> Self {
        let value = cx.new(|_cx| value);
        Self {
            value,
            current_path: Default::default(),
            by_path: Default::default(),
        }
    }

    /// Retrieve the state associated with the current traversal location
    pub fn path_state(&mut self) -> &mut PerPathState {
        self.by_path
            .entry(self.current_path.clone())
            .or_insert_with(|| PerPathState { open: true })
    }

    /// Navigate into the object via a number or key index
    pub fn visiting<R>(
        &mut self,
        index: JsonIndex,
        window: &mut Window,
        cx: &mut Context<Self>,
        f: impl Fn(&mut Self, &mut Window, &mut Context<Self>) -> R,
    ) -> R
    where
        R: 'static,
    {
        self.current_path.push_index(index.clone());
        let res = f(self, window, cx);
        self.current_path.pop();
        res
    }
}

impl Render for ObjectWidget {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let value = self.value.read(cx).clone();
        div()
            //
            .p_2()
            .child(
                div()
                    //
                    .p_2()
                    .bg(cx.theme().colors().element_background)
                    .rounded_lg()
                    .child(
                        div()
                            //
                            .w_full()
                            .p_2()
                            .child("Object Header".to_string()),
                    )
                    .child(self.render_value(&value, window, cx)),
            )
    }
}

impl ObjectWidget {
    fn render_value(
        &mut self,
        value: &Value,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let it = div();
        // let it = div().size_full().debug();
        let it = match value {
            Value::Null => {
                //
                it.child("null").into_any_element()
            }
            Value::Bool(b) => {
                //
                it.child(if *b { "true" } else { "false" })
                    .into_any_element()
            }
            Value::Number(number) => {
                //
                it.child(format!("{number}")).into_any_element()
            }
            Value::String(string) => {
                //
                it.child(string.to_string()).into_any_element()
            }
            Value::Array(values) => {
                //
                it.child(self.render_list(values, window, cx))
                    .into_any_element()
            }
            Value::Object(map) => {
                //
                it.child(self.render_map(map, window, cx))
                    .into_any_element()
            }
        };

        it
    }

    fn render_map(
        &mut self,
        map: &Map<String, Value>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_color = cx.theme().colors().ghost_element_active;
        let hover_bg_color = cx.theme().colors().ghost_element_hover;
        let hover_border_color = cx.theme().colors().border.opacity(1.0);

        div()
            //
            .flex()
            .flex_col()
            .children(map.iter().enumerate().map(|(i, (key, value))| {
                // foreach kv
                div()
                    // TODO better IDs
                    .id(SharedString::from(format!("object_widget-{i}")))
                    .w_full()
                    .flex()
                    .flex_row()
                    .active(|style| style.bg(active_color))
                    .hover(|style| style.bg(hover_bg_color).border_color(hover_border_color))
                    .child(
                        div()
                            //
                            .p_2()
                            .flex_1()
                            .child(format!("Key: {key}")),
                    )
                    .child(div().p_2().flex_1().child({
                        self.visiting(
                            JsonIndex::Key(key.to_string()),
                            window,
                            cx,
                            |this, window, cx| {
                                //
                                this.render_value(value, window, cx)
                            },
                        )
                    }))
            }))
    }

    fn render_list(
        &mut self,
        values: &Vec<Value>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            //
            .flex()
            .flex_col()
            .children(values.iter().enumerate().map(|(i, value)| {
                //
                div()
                    //
                    .p_2()
                    .flex()
                    .flex_row()
                    .child(div().child(format!("Index: {}", i)))
                    .child(
                        self.visiting(JsonIndex::Number(i), window, cx, |this, window, cx| {
                            this.render_value(value, window, cx)
                        }),
                    )
            }))
            .into_any_element()
    }
}
