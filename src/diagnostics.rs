use std::time::Duration;

use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin,
    },
    prelude::*,
    time::common_conditions::once_after_delay,
};
use bevy_enhanced_input::prelude::Started;

use crate::userinput::Diag;

// From https://bevy.org/examples/diagnostics/log-diagnostics/
// And https://bevy.org/examples/diagnostics/enabling-disabling-diagnostic/
// And https://bevy.org/examples/diagnostics/custom-diagnostic/
pub struct DiagnosticsPlugin;

/* All diagnostics should have a unique DiagnosticPath.
const SYSTEM_ITERATION_COUNT: DiagnosticPath = DiagnosticPath::const_new("system_iteration_count");

fn my_system(mut diagnostics: Diagnostics) {
    // Add a measurement of 10.0 for our diagnostic each time this system runs.
    diagnostics.add_measurement(&SYSTEM_ITERATION_COUNT, || 10.0);
}
*/

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Adds a system that prints diagnostics to the console
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // Adds cpu and memory usage diagnostics for systems and the entire game process.
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            // Forwards various diagnostics from the render app to the main app.
            // These are pretty verbose but can be useful to pinpoint performance issues.
            bevy::render::diagnostic::RenderDiagnosticsPlugin,
            // Adds a system that prints bevy_framepace additionnal diagnostics
            bevy_framepace::debug::DiagnosticsPlugin,
        ))
        // Diagnostics must be initialized before measurements can be added.
        // .register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"))
        .add_systems(Startup, Self::setup)
        .add_systems(
            Update,
            Self::disable.run_if(once_after_delay(Duration::from_secs(5))),
        )
        .add_observer(Self::toggle_diag);
    }
}

impl DiagnosticsPlugin {
    fn setup() {
        info!("registered DiagnosticsPlugin");
    }

    fn disable(mut store: ResMut<DiagnosticsStore>) {
        for diag in store.iter_mut() {
            if *diag.path() == FrameTimeDiagnosticsPlugin::FPS {
                continue;
            }
            diag.is_enabled = false;
        }
    }

    fn toggle_diag(_trigger: Trigger<Started<Diag>>, mut store: ResMut<DiagnosticsStore>) {
        info!("toggling diagnostics");

        for diag in store.iter_mut() {
            if *diag.path() == FrameTimeDiagnosticsPlugin::FPS {
                continue;
            }
            diag.is_enabled = !diag.is_enabled;
        }
    }
}
