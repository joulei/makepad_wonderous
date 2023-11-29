use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    ArtifactsScreen = <View> {
        width: Fill, height: Fill
        flow: Right,

        show_bg: true,
        draw_bg: {
            color: #004
        }
    }
}
