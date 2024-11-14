use crate::context::{Context, NavigationAction};

pub use adw::prelude::*;
pub use std::error::Error;
pub use crate::context::Action;

pub trait HasWidget<W: IsA<gtk::Widget> + Sized> {
    fn widget(&self) -> &W;
}

impl <W: IsA<gtk::Widget>>std::borrow::Borrow<W> for dyn HasWidget<W> {
    fn borrow(&self) -> &W { self.widget() }
}

impl <W: IsA<gtk::Widget>>std::convert::AsRef<W> for dyn HasWidget<W> {
    fn as_ref(&self) -> &W { self.widget() }
}

pub trait LifeCycle<A: Action> {
    fn activate(&self, action: A, context: &Context);
    fn deactivate(&self) {}
}

pub trait PageContent: LifeCycle<NavigationAction> + HasWidget<gtk::Widget> {
    fn title(&self) -> &str;
}

pub trait Propagator<A: Action> {
    fn propagate(&self, action: A, context: &Context);
}