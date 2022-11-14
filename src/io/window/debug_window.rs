use bevy::prelude::*;
use bevy::render::once_cell::sync::Lazy;
use bevy::render::render_graph::RenderGraph;
use bevy::render::RenderApp;
use bevy::window::{CreateWindow, PresentMode, WindowId};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use iyes_loopless::prelude::ConditionSet;

use crate::AppState;

static SECOND_WINDOW_ID: Lazy<WindowId> = Lazy::new(WindowId::new);
const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";

pub struct DebugWindowPlugin;

impl Plugin for DebugWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_plugin(WorldInspectorPlugin::new());

        let render_app = app.sub_app_mut(RenderApp);
        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        bevy_egui::setup_pipeline(
            &mut graph,
            bevy_egui::RenderGraphConfig {
                window_id: *SECOND_WINDOW_ID,
                egui_pass: SECONDARY_EGUI_PASS,
            },
        );

        app.add_startup_system(create_new_window)
            .add_startup_system(configure_egui_look)
            .add_system_set(ConditionSet::new().run_in_state(AppState::Game).into());
    }
}

fn create_new_window(
    mut create_window_events: EventWriter<CreateWindow>,
    mut params: ResMut<WorldInspectorParams>,
) {
    params.window = *SECOND_WINDOW_ID;
    // sends out a "CreateWindow" event, which will be received by the windowing backend
    create_window_events.send(CreateWindow {
        id: *SECOND_WINDOW_ID,
        descriptor: WindowDescriptor {
            width: 800.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            title: "Hex Debugging Window".to_string(),
            ..Default::default()
        },
    });
}

// This is a global look for egui
fn configure_egui_look(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}
