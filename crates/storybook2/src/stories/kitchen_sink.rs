use std::marker::PhantomData;

use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::story::Story;
use crate::story_selector::{ComponentStory, ElementStory};

#[derive(Element)]
pub struct KitchenSinkStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> KitchenSinkStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let element_stories = ElementStory::iter().map(|selector| selector.story());
        let component_stories = ComponentStory::iter().map(|selector| selector.story(cx)).collect::<Vec<_>>();

        Story::container(cx)
            .overflow_y_scroll(ScrollState::default())
            .child(Story::title(cx, "Kitchen Sink"))
            .child(Story::label(cx, "Elements"))
            .child(div().flex().flex_col().children_any(element_stories))
            .child(Story::label(cx, "Components"))
            .child(div().flex().flex_col().children_any(component_stories))
            // Add a bit of space at the bottom of the kitchen sink so elements
            // don't end up squished right up against the bottom of the screen.
            .child(div().p_4())
    }
}
