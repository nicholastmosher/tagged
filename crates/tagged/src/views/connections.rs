// The Connections UI will initially be Iroh peer connections
//
// This UI is in charge of:
// - Displaying a list of connected peers
// - Allowing adding new peers
// - Allowing removing peers

use zed::unstable::{
    gpui::{AppContext as _, Entity},
    ui::{App, Context, IntoElement, ParentElement as _, Render, Styled, Window, div},
};

pub fn init(cx: &mut App) {
    //
}

pub struct ConnectionsUi {
    //
}

impl ConnectionsUi {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn build(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(cx))
    }
}

impl Render for ConnectionsUi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            //
            .text_lg()
            .child("Connections")
    }
}
