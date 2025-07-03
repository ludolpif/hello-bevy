use bevy::prelude::*;

// note: Debug was added to make info!("{:?}", csc) work
// see: https://doc.rust-lang.org/std/fmt/#formatting-traits
#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct ColorSourceComponent {
    pub native_size: Vec2,
    //pub transform: Transform,
    //pub color: Color,
    /* A `Duration` field that should never be serialized to the scene file, so we skip it.
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
    */
}

pub struct ColorSourcePlugin;

impl Plugin for ColorSourcePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ColorSourceComponent>()
            .add_systems(Startup, Self::setup);
    }
}

impl ColorSourcePlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        info!("registered ColorSourcePlugin");
        let size = Vec2 { x: 50.0, y: 50.0 };
        let shape = meshes.add(Rectangle::from_size(size));
        let color = Color::hsl(0.0, 0.95, 0.7);
        let material = materials.add(color);
        commands.spawn((
            Name::new("ColorSource"),
            ColorSourceComponent { native_size: size },
            Mesh2d(shape),
            MeshMaterial2d(material),
            Transform::IDENTITY,
        ));
    }
}
