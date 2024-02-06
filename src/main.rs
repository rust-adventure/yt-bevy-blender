use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::*;

fn main() {
    App::new()
        .register_type::<Coin>()
        .add_plugins((
            DefaultPlugins,
            ExportRegistryPlugin::default(),
            ComponentsFromGltfPlugin::default(),
        ))
        .add_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<LevelAssets>(),
        )
        .add_systems(OnEnter(MyStates::Next), start_level)
        .add_systems(Update, rotate_coins)
        .run();
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Coin;

fn rotate_coins(
    mut query: Query<&mut Transform, With<Coin>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "level.glb")]
    level: Handle<Gltf>,
}

fn start_level(
    mut commands: Commands,
    assets: Res<LevelAssets>,
    models: Res<Assets<bevy::gltf::Gltf>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    let my_gltf = models.get(assets.level.clone()).unwrap();

    commands.spawn((
        SceneBundle {
            scene: my_gltf.scenes[0].clone(),
            ..default()
        },
        Name::new("Level1"),
    ));
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
