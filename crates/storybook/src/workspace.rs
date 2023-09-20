use crate::{
    theme::theme,
    ui::{chat_panel, project_panel, status_bar, tab_bar, title_bar},
};
use gpui2::{
    elements::{div, div::ScrollState},
    style::StyleHelpers,
    Element, IntoElement, ParentElement, ViewContext,
};

#[derive(Element, Default)]
struct WorkspaceElement {
    left_scroll_state: ScrollState,
    right_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
}

pub fn workspace<V: 'static>() -> impl Element<V> {
    WorkspaceElement::default()
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.lowest.base.default.background)
            .child(title_bar())
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(project_panel(self.left_scroll_state.clone()))
                    .child(
                        div()
                            .h_full()
                            .flex_1()
                            .fill(theme.highest.base.default.background)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .child(tab_bar(self.tab_bar_scroll_state.clone())),
                            ),
                    )
                    .child(chat_panel(self.right_scroll_state.clone())),
            )
            .child(status_bar())
    }
}
