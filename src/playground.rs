use bevy::prelude::*;

// From: https://bevy.org/examples/application/logs/
// And: https://bevy.org/examples/application/plugin/
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (HelloPlugin::hello_world, HelloPlugin::display_title),
        )
        .add_systems(Update, HelloPlugin::dont_spam);
    }
}

impl HelloPlugin {
    fn hello_world() {
        trace!("tracing");
        debug!("solving issues...");
        info!("hello :)");
        warn!("spooky warning");
        error!("scary error");
    }
    fn dont_spam() {
        warn_once!("emit this only once, even if the system is called at every frame");
    }
    fn display_title(mut commands: Commands) {
        commands.spawn(Camera2d);

        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Text::new("Hello World"),
                    TextFont {
                        font_size: 130.0,
                        ..default()
                    },
                ),
                (
                    Text::new("and all the folks"),
                    TextFont {
                        font_size: 100.0,
                        ..default()
                    },
                )
            ],
        ));
    }
}
