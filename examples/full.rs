use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use bevy_ninepatch::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins((
            DefaultPlugins,
            NinePatchPlugin::<Content>::default(),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(Msaa::Off)
        // Add the `NinePatchPlugin` plugin
        // Adds a system that prints diagnostics to the console
        .add_systems(Startup, setup)
        .add_systems(Update, (set_content, update_size))
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
) {
    // load the assets
    let cornered_panel_texture_handle = asset_server.load("metalPanel_yellowCorner.png");

    let panel_nine_patch_handle = nine_patches.add(NinePatchBuilder::from_patches(vec![
        vec![
            // top left corner patch
            Patch {
                original_size: IVec2::new(30, 35),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
            // top middle-left patch. This patch width can grow, and will contain the content for
            // `PanelContent::Title`
            Patch {
                original_size: IVec2::new(15, 35),
                target_width: Val::Percent(30.),
                target_height: Val::ZERO,
                content: Some(Content::Title),
            },
            // top middle patch. In the original PNG, it's the yellow titled part
            Patch {
                original_size: IVec2::new(25, 35),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
            // top middle-right patch. This patch width can grow
            Patch {
                original_size: IVec2::new(20, 35),
                target_width: Val::Percent(70.),
                target_height: Val::ZERO,
                content: None,
            },
            // top right corner
            Patch {
                original_size: IVec2::new(10, 35),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
        ],
        vec![
            // left border. This patch height can grow
            Patch {
                original_size: IVec2::new(10, -45),
                target_width: Val::ZERO,
                target_height: Val::Percent(100.),
                content: None,
            },
            // center. This patch can grow both in height and width, and will contain `PanelContent::Body`
            Patch {
                original_size: IVec2::new(-20, -45),
                target_width: Val::Percent(100.),
                target_height: Val::Percent(100.),
                content: Some(Content::Content),
            },
            // right border. This patch height can grow
            Patch {
                original_size: IVec2::new(10, -45),
                target_width: Val::ZERO,
                target_height: Val::Percent(100.),
                content: None,
            },
        ],
        vec![
            // bottom left corner
            Patch {
                original_size: IVec2::new(10, 10),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
            // bottom middle. This patch width can grow
            Patch {
                original_size: IVec2::new(-20, 10),
                target_width: Val::Percent(100.),
                target_height: Val::ZERO,
                content: None,
            },
            // bottom right corner
            Patch {
                original_size: IVec2::new(10, 10),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
        ],
    ]));

    commands.spawn((
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(900.),
                height: Val::Px(600.),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: panel_nine_patch_handle,
                texture: cornered_panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
        UiElement::Panel,
    ));

    commands.spawn(Camera2dBundle::default());
}

fn set_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
    mut patch_content: Query<(Entity, &mut NinePatchContent<Content>)>,
    ui_element_query: Query<&UiElement>,
    mut font: Local<Handle<Font>>,
) {
    *font = asset_server.load("Kenney Future Narrow.ttf");

    for (entity, mut nine_patch_content) in &mut patch_content.iter_mut() {
        if !nine_patch_content.loaded {
            match (
                *ui_element_query
                    .get_component::<UiElement>(nine_patch_content.parent)
                    .unwrap(),
                &nine_patch_content.content,
            ) {
                (UiElement::Panel, Content::Content) => {
                    let panel_texture_handle: Handle<Image> =
                        asset_server.load("glassPanel_corners.png");

                    // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
                    let nine_patch_handle = nine_patches.add(
                        NinePatchBuilder::by_margins_with_content(20, 20, 20, 20, Content::Content),
                    );

                    let content_entity = commands
                        .spawn((
                            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
                            // of this entity
                            NinePatchBundle {
                                style: Style {
                                    margin: UiRect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(850.),
                                    height: Val::Px(550.),
                                    ..Default::default()
                                },
                                nine_patch_data: NinePatchData {
                                    nine_patch: nine_patch_handle,
                                    texture: panel_texture_handle,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            UiElement::InnerPanel,
                        ))
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
                (UiElement::Panel, Content::Title) => {
                    let content_entity = commands
                        .spawn(
                            TextBundle::from_section(
                                "Example  Title",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 25.0,
                                    color: Color::BLUE,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    left: Val::ZERO,
                                    right: Val::Auto,
                                    top: Val::Px(8.),
                                    bottom: Val::Auto,
                                },
                                ..Default::default()
                            }),
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
                (UiElement::InnerPanel, _) => {
                    // prepare the button
                    let button_texture_handle = asset_server.load("blue_button02.png");
                    let button_nine_patch_handle = nine_patches.add(
                        NinePatchBuilder::by_margins_with_content(5, 10, 6, 6, Content::Content),
                    );

                    let button_cancel_entity = commands
                        .spawn((
                            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
                            // of this entity
                            NinePatchBundle {
                                style: Style {
                                    margin: UiRect {
                                        left: Val::Px(0.),
                                        right: Val::Auto,
                                        top: Val::Auto,
                                        bottom: Val::Px(0.),
                                    },
                                    width: Val::Px(300.),
                                    height: Val::Px(80.),

                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                nine_patch_data: NinePatchData {
                                    nine_patch: button_nine_patch_handle.clone(),
                                    texture: button_texture_handle.clone(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            UiElement::ButtonCancel,
                        ))
                        .id();

                    let button_ok_entity = commands
                        .spawn((
                            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
                            // of this entity
                            NinePatchBundle {
                                style: Style {
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Px(0.),
                                        top: Val::Auto,
                                        bottom: Val::Px(0.),
                                    },
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(300.),
                                    height: Val::Px(80.),
                                    ..Default::default()
                                },
                                nine_patch_data: NinePatchData {
                                    nine_patch: button_nine_patch_handle,
                                    texture: button_texture_handle,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            UiElement::ButtonOK,
                        ))
                        .id();

                    commands
                        .entity(entity)
                        .push_children(&[button_cancel_entity, button_ok_entity]);
                    nine_patch_content.loaded = true;
                }
                (UiElement::ButtonOK, _) => {
                    let content_entity = commands
                        .spawn(
                            TextBundle::from_section(
                                "OK",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 50.0,
                                    color: Color::GREEN,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    left: Val::Px(110.),
                                    right: Val::Auto,
                                    top: Val::Px(10.),
                                    bottom: Val::Auto,
                                },
                                ..Default::default()
                            }),
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
                (UiElement::ButtonCancel, _) => {
                    let content_entity = commands
                        .spawn(
                            TextBundle::from_section(
                                "CANCEL",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 50.0,
                                    color: Color::RED,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    left: Val::Px(50.),
                                    right: Val::Auto,
                                    top: Val::Px(10.),
                                    bottom: Val::Auto,
                                },
                                ..Default::default()
                            }),
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, TypePath)]
enum Content {
    Title,
    Content,
}

#[derive(Clone, Copy, Component)]
enum UiElement {
    Panel,
    InnerPanel,
    ButtonOK,
    ButtonCancel,
}

// by changing the component `Style.size`, the 9-Patch UI element will be resized
fn update_size(time: Res<Time>, mut query: Query<(&mut Style, &UiElement)>) {
    for (mut style, panel) in query.iter_mut() {
        let (x, y) = time.elapsed_seconds().sin_cos();

        match panel {
            UiElement::Panel => {
                style.width = Val::Px((900. + 50. * x as f32).ceil());
                style.height = Val::Px((600. + 50. * y as f32).ceil());
            }
            UiElement::InnerPanel => {
                style.width = Val::Px((850. + 50. * x as f32).ceil());
                style.height = Val::Px((550. + 50. * y as f32).ceil());
            }
            UiElement::ButtonOK => style.width = Val::Px((300. + 50. * x as f32).ceil()),
            UiElement::ButtonCancel => style.height = Val::Px((90. + 10. * y as f32).ceil()),
        }
    }
}
