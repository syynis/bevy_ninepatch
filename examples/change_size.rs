use bevy::prelude::*;

use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData, NinePatchPlugin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins((DefaultPlugins, NinePatchPlugin::<()>::default()))
        // Add the `NinePatchPlugin` plugin
        .add_systems(Startup, setup)
        // this system will change the size depending on time elapsed since startup
        .add_systems(Update, update_size)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    let panel_texture_handle = asset_server.load("glassPanel_corners.png");

    // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(50.),
                height: Val::Px(50.),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(Camera2dBundle::default());
}

// by changing the component `Style.size`, the 9-Patch UI element will be resized
fn update_size(time: Res<Time>, mut query: Query<&mut Style, With<NinePatchData<()>>>) {
    for mut style in query.iter_mut() {
        let (x, y) = time.elapsed_seconds().sin_cos();

        style.width = Val::Px((250. + 200. * x as f32).ceil());
        style.height = Val::Px((250. + 200. * y as f32).ceil());
    }
}
