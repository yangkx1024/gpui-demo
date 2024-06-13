use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gpui::*;

#[derive(Debug, Clone, IntoElement)]
pub struct ListItem {
    title: SharedString,
    subtitle: SharedString,
}

impl RenderOnce for ListItem {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x3a3a3a))
            .rounded_md()
            .hover(|s| s.bg(rgb(0x2a2a2a)))
            .p_2()
            .m_2()
            .items_start()
            .text_color(rgb(0xffffff))
            .text_xl()
            .child(self.title.clone())
            .child(self.subtitle.clone())
    }
}

impl ListItem {
    pub fn new(title: String, subtitle: String) -> Self {
        ListItem {
            title: title.into(),
            subtitle: subtitle.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    items: Rc<RefCell<Vec<ListItem>>>,
}

pub struct Main {
    state_model: Model<State>,
    list_state: ListState,
}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state_model = self.state_model.clone();
        let button = div()
            .flex()
            .p_2()
            .bg(rgb(0x2a2a2a))
            .bg(rgb(0x3a3a3a))
            .rounded_md()
            .hover(|s| s.bg(rgb(0x2a2a2a)))
            .text_color(rgb(0xffffff))
            .text_xl()
            .cursor(CursorStyle::PointingHand)
            .child("Add Item")
            .on_mouse_down(MouseButton::Left, move |_, cx| {
                cx.update_model(&state_model, |state: &mut State, cx| {
                    let count = state.items.borrow().len();
                    let new_item = ListItem::new(
                        format!("Item {}", count),
                        "Subtitle".to_string(),
                    );
                    state.items.borrow_mut().push(new_item);
                    cx.notify();
                })
            });
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(list(self.list_state.clone()).w_full().h_full())
            .child(
                div()
                    .flex()
                    .w_full()
                    .py_2()
                    .justify_center()
                    .items_center()
                    .child(button)
            )
    }
}

impl Main {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let state_model = cx.new_model(|_cx| State { items: Rc::new(RefCell::new(vec![])) });

            let list_state = ListState::new(0, ListAlignment::Bottom, Pixels(20.), |_, _| {
                div().into_any_element()
            });
            cx.observe(&state_model, |this: &mut Main, model, cx| {
                let state = model.read(cx).clone();
                let scroll_offset = this.list_state.logical_scroll_top();
                let len = state.items.borrow().len();
                this.list_state = ListState::new(
                    len,
                    ListAlignment::Bottom,
                    Pixels(20.),
                    move |index, _cx| div().child(state.items.borrow()[index].clone()).into_any_element(),
                );
                this.list_state.scroll_to(scroll_offset);
                cx.notify();
            }).detach();
            Self::load_data(cx, state_model.clone());
            Self {
                state_model,
                list_state,
            }
        })
    }

    fn load_data(cx: &mut ViewContext<Self>, state_model: Model<State>) {
        cx.spawn(|_, mut cx| async move {
            loop {
                let background_executor = cx.background_executor().clone();
                background_executor.spawn(async {
                    // Simulate network request
                    Timer::after(Duration::from_millis(100)).await
                }).await;
                let result = cx.update_model(&state_model, |state, cx| {
                    let count = state.items.borrow().len();
                    let new_item = ListItem::new(
                        format!("Item {}", count),
                        "Subtitle".to_string(),
                    );
                    state.items.borrow_mut().push(new_item);
                    cx.notify();
                });
                if let Err(e) = result {
                    eprintln!("{:#}", e);
                    break;
                }
            }
        }).detach();
    }
}
