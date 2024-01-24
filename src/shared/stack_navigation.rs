use crate::shared::stack_view_action::StackViewAction;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::FillerX;

    SimpleHeaderContent = <View> {
        width: Fill, height: Fit
        flow: Right, align: {x: 0.5, y: 0.5}

        <FillerX> {}

        title_container = <View> {
            width: Fill, height: Fit
            align: {x: 0.5, y: 0.5}

            title = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    color: #1f1b18,
                    text_style: <REGULAR_TEXT>{},
                },
                text: ""
            }
        }
    }

    SimpleHeader = <View> {
        width: Fill , height: Fit, margin: 0
        padding: {bottom: 7., top: 50.}, align: {x: 0.5, y: 0.0}, spacing: 0.0, flow: Overlay
        show_bg: true
        draw_bg: {
            color: #1f1b18
        }

        content = <SimpleHeaderContent> {}
    }

    HeaderWithLeftActionButton = <SimpleHeader> {
        content = {
            flow: Overlay

            button_container = <View> {
                left_button = <Button> {
                    width: Fit, height: 68
                    icon_walk: {width: 20, height: 68}
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return #1f1b18
                        }
                    }
                    draw_icon: {
                        color: #fff;
                    }
                }

            }
        }
    }

    Header = <HeaderWithLeftActionButton> {
        content = {
            title_container = {
                title = {
                    text: "s"
                }
            }

            button_container = {
                left_button = {
                    width: Fit
                    icon_walk: {width: 10}
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/back.svg")
                    }
                }
            }
        }
    }

    StackNavigationView = {{StackNavigationView}} {
        visible: false
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: #1f1b18
        }

        header = <Header> {}

        // TBD Adjust this based on actual screen size
        offset: 400.0

        animator: {
            slide = {
                default: hide,
                hide = {
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    from: {all: Forward {duration: 0.3}}
                    // Bug: Constants are not working as part of an live state value
                    apply: {offset: 400.0}
                }

                show = {
                    ease: ExpDecay {d1: 0.82, d2: 0.95}
                    from: {all: Forward {duration: 0.3}}
                    apply: {offset: 0.0}
                }
            }
        }
    }

    StackNavigation = {{StackNavigation}} {
        width: Fill, height: Fill
        flow: Overlay

        root_view = <View> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct StackNavigationView {
    #[deref]
    view: View,

    #[live]
    offset: f64,

    #[animator]
    animator: Animator,
}

impl Widget for StackNavigationView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).is_animating() {
            self.view.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if self.animator.animator_in_state(cx, id!(slide.hide))
            && !self.animator.is_track_animating(cx, id!(slide))
        {
            self.apply_over(cx, live! {visible: false});
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(
            cx,
            scope,
            walk.with_abs_pos(DVec2 {
                x: self.offset,
                y: 0.,
            }),
        )
    }
}

impl WidgetMatchEvent for StackNavigationView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(id!(left_button)).clicked(&actions) {
            self.animator_play(cx, id!(slide.hide));
        }
    }
}

impl StackNavigationViewRef {
    pub fn show(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.apply_over(cx, live! {visible: true});
            inner.animator_play(cx, id!(slide.show));
        }
    }

    pub fn is_showing(&self, cx: &mut Cx) -> bool {
        if let Some(inner) = self.borrow() {
            inner.animator.animator_in_state(cx, id!(slide.show))
                || inner.animator.is_track_animating(cx, id!(slide))
        } else {
            false
        }
    }

    pub fn is_animating(&self, cx: &mut Cx) -> bool {
        if let Some(inner) = self.borrow() {
            inner.animator.is_track_animating(cx, id!(slide))
        } else {
            false
        }
    }
}

#[derive(Default)]
enum ActiveStackView {
    #[default]
    None,
    Active(LiveId),
}

#[derive(Live, LiveRegisterWidget, WidgetRef)]
pub struct StackNavigation {
    #[deref]
    view: View,

    #[rust]
    active_stack_view: ActiveStackView,
}

impl LiveHook for StackNavigation {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.active_stack_view = ActiveStackView::None;
    }
}

impl Widget for StackNavigation {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for widget_ref in self.get_active_views(cx).iter() {
            widget_ref.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        for widget_ref in self.get_active_views(cx.cx).iter() {
            widget_ref.draw_walk(cx, scope, walk)?;
        }
        DrawStep::done()
    }
}

impl WidgetNode for StackNavigation {
    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        for widget_ref in self.get_active_views(cx).iter() {
            widget_ref.redraw(cx);
        }
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }
}

impl StackNavigation {
    pub fn show_stack_view_by_id(&mut self, stack_view_id: LiveId, cx: &mut Cx) {
        if let ActiveStackView::None = self.active_stack_view {
            let mut stack_view_ref = self.stack_navigation_view(&[stack_view_id]);
            stack_view_ref.show(cx);
            self.active_stack_view = ActiveStackView::Active(stack_view_id);
            self.redraw(cx);
        }
    }

    fn get_active_views(&mut self, cx: &mut Cx) -> Vec<WidgetRef> {
        match self.active_stack_view {
            ActiveStackView::None => {
                vec![self.view.widget(id!(root_view))]
            }
            ActiveStackView::Active(stack_view_id) => {
                let stack_view_ref = self.stack_navigation_view(&[stack_view_id]);
                let mut views = vec![];

                if stack_view_ref.is_showing(cx) {
                    if stack_view_ref.is_animating(cx) {
                        views.push(self.view.widget(id!(root_view)));
                    }
                    views.push(stack_view_ref.0.clone());
                    views
                } else {
                    self.active_stack_view = ActiveStackView::None;
                    vec![self.view.widget(id!(root_view))]
                }
            }
        }
    }
}

impl StackNavigationRef {
    pub fn show_stack_view_by_id(&mut self, stack_view_id: LiveId, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_stack_view_by_id(stack_view_id, cx);
        }
    }

    pub fn handle_stack_view_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        destinations: &HashMap<StackViewAction, LiveId>,
    ) {
        for action in actions {
            let stack_view_action = action.as_widget_action().cast();
            if let Some(stack_view_id) = destinations.get(&stack_view_action) {
                self.show_stack_view_by_id(*stack_view_id, cx);
                break;
            }
        }
    }
}
