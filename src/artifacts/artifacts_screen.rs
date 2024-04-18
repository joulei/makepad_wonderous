use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    import crate::artifacts::artifacts_carrousel::*;

    BACKGROUND_ITEM_COLOR = #333

    ArtifactsScreen = {{ArtifactsScreen}} {
        width: Fill, height: Fill
        flow: Overlay,
        align: {x: 0.5, y: 0.0},

        <Label> {
            margin: { top: 40.0 }
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 10},
                color: #fff
            }
            text: "ARTIFACTS"
        }

        <ArtifactsCarrousel> {
            margin: { top: 30.0 }
        }

        <View> {
            height: Fill,
            margin: 20,
            flow: Down,
            
            <View> { height: Fill, width: 1 }

            browse_artifacts_button = <Button> {
                width: Fill,
                height: 50,
                text: "BROWSE ALL ARTIFACTS",
                draw_text: {
                    text_style: {
                        font_size: 9.0
                    }

                    fn get_color(self) -> vec4 {
                        return #fff
                    }
                }

                draw_bg: { bodytop: (BACKGROUND_ITEM_COLOR), bodybottom: (BACKGROUND_ITEM_COLOR) }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ArtifactsScreen {
    #[deref]
    view: View,
}

impl Widget for ArtifactsScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for ArtifactsScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self
            .button(id!(browse_artifacts_button))
            .clicked(&actions)
        {
            let widget_uid = self.widget_uid();
            cx.widget_action(
                widget_uid,
                &scope.path,
                StackNavigationAction::NavigateTo(live_id!(artifact_gallery_view))
            );
        }
    }
}
