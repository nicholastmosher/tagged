use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use zed::unstable::{
    gpui::{AppContext, Entity},
    ui::App,
};

use crate::willow::WillowExt;

// Need a name for "types which "
pub trait Willowize: 'static + JsonSchema + Serialize + for<'de> Deserialize<'de> {
    //
}

pub trait ThingHandle {
    //
    fn eg_write_to_willow(&self, cx: &mut App) {}
}

// FromContext
// Like App -> Context<T>
// But App -> WillowContext<T>
// impl<'a> From<&'a mut App> for &'a mut WillowContext<T> { }
// meh no

// Extension traits for entities that do whatever this is
impl<T: Willowize> ThingHandle for Entity<T> {
    fn eg_write_to_willow(&self, cx: &mut App) {
        // let value = self.read(cx);
        // let serialized = serde_json::to_string(value).unwrap();
        // let entry = todo!();
        cx.willow()
            //
            .todo_write_to_willow(self, cx);
    }
}

fn doit<T: Willowize>(it: T) {
    let it = serde_json::to_vec(&it).unwrap();
    let it = serde_json::from_slice::<T>(&*it).unwrap();
    let schema = schema_for!(T);
}
