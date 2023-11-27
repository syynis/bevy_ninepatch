use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    ui::FocusPolicy,
};

/// Describe a patch in the original image, how it should grow and if it can have content
#[derive(Debug, Clone, TypePath)]
pub struct Patch<T: Clone + Send + Sync + 'static> {
    /// Size of the patch in the original image
    pub original_size: IVec2,
    /// Width of the patch rendered
    pub target_width: Val,
    /// Height of the patch rendered
    pub target_height: Val,
    /// Does this patch can contain content
    pub content: Option<T>,
}

/// Holds the patches of a nine patch texture
#[derive(Debug, TypePath, TypeUuid, Asset)]
#[uuid = "ec4c1e8f-d0fb-42f4-8bcd-49b71cea3a7d"]
pub struct NinePatchBuilder<T: Clone + Send + Sync + Eq + std::hash::Hash + TypePath + 'static = ()>
{
    /// Patches for a nine patch texture. See example `full.rs` on how to use directly
    pub patches: Vec<Vec<Patch<T>>>,
    pub(crate) patch_textures: Option<Vec<Handle<Image>>>,
    pub(crate) original_texture: Option<Handle<Image>>,
}

impl<T: Clone + Send + Sync + Eq + std::hash::Hash + TypePath + 'static> NinePatchBuilder<T> {
    /// Create a `NinePatchBuilder` from it's patches
    pub fn from_patches(patches: Vec<Vec<Patch<T>>>) -> Self {
        Self {
            patches,
            patch_textures: None,
            original_texture: None,
        }
    }
}

impl<T: Clone + Send + Sync + Default + Eq + std::hash::Hash + TypePath + 'static>
    NinePatchBuilder<T>
{
    /// Create a simple nine patch split by creating fixed patch for the margins, and growing patches inside
    pub fn by_margins(
        top_margin: u32,
        bottom_margin: u32,
        left_margin: u32,
        right_margin: u32,
    ) -> Self {
        Self::by_margins_with_content(
            top_margin,
            bottom_margin,
            left_margin,
            right_margin,
            T::default(),
        )
    }
}

impl<T: Clone + Send + Sync + Eq + std::hash::Hash + TypePath + 'static> NinePatchBuilder<T> {
    /// Create a simple nine patch split by creating fixed patch for the margins, and growing patches inside
    pub fn by_margins_with_content(
        top_margin: u32,
        bottom_margin: u32,
        left_margin: u32,
        right_margin: u32,
        content: T,
    ) -> Self {
        let top = vec![
            Patch {
                original_size: IVec2::new(left_margin as i32, top_margin as i32),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
            Patch {
                original_size: IVec2::new(
                    -(left_margin as i32) - right_margin as i32,
                    top_margin as i32,
                ),
                target_width: Val::Auto,
                target_height: Val::ZERO,
                content: None,
            },
            Patch {
                original_size: IVec2::new(right_margin as i32, top_margin as i32),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
        ];
        let middle = vec![
            Patch {
                original_size: IVec2::new(
                    left_margin as i32,
                    -(top_margin as i32) - bottom_margin as i32,
                ),
                target_width: Val::ZERO,
                target_height: Val::Auto,
                content: None,
            },
            Patch {
                original_size: IVec2::new(
                    -(left_margin as i32) - right_margin as i32,
                    -(top_margin as i32) - bottom_margin as i32,
                ),
                target_width: Val::Auto,
                target_height: Val::Auto,
                content: Some(content),
            },
            Patch {
                original_size: IVec2::new(
                    right_margin as i32,
                    -(top_margin as i32) - bottom_margin as i32,
                ),
                target_width: Val::ZERO,
                target_height: Val::Auto,
                content: None,
            },
        ];
        let bottom = vec![
            Patch {
                original_size: IVec2::new(left_margin as i32, bottom_margin as i32),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
            Patch {
                original_size: IVec2::new(
                    -(left_margin as i32) - right_margin as i32,
                    bottom_margin as i32,
                ),
                target_width: Val::Auto,
                target_height: Val::ZERO,
                content: None,
            },
            Patch {
                original_size: IVec2::new(right_margin as i32, bottom_margin as i32),
                target_width: Val::ZERO,
                target_height: Val::ZERO,
                content: None,
            },
        ];
        Self {
            patches: vec![top, middle, bottom],
            patch_textures: None,
            original_texture: None,
        }
    }
}

fn to_width(patch: IVec2, total: Extent3d) -> u32 {
    if patch.x > 0 {
        patch.x as u32
    } else {
        (total.width as i32 + patch.x) as u32
    }
}
fn to_height(patch: IVec2, total: Extent3d) -> u32 {
    if patch.y > 0 {
        patch.y as u32
    } else {
        (total.height as i32 + patch.y) as u32
    }
}

impl<T: Clone + Send + Sync + Eq + std::hash::Hash + TypePath + 'static> NinePatchBuilder<T> {
    /// Apply a `NinePatchBuilder` to a texture to get a `NinePatch` ready to be added to entities. This will split
    /// the given texture according to the patches.
    pub fn apply(
        &mut self,
        texture_handle: &Handle<Image>,
        textures: &mut Assets<Image>,
    ) -> NinePatch<T> {
        let (texture_size, texture_data) = {
            let t = textures
                .get(texture_handle)
                .expect("could not get texture from handle");
            (t.texture_descriptor.size, &t.data)
        };
        let mut textures_to_add = vec![];
        if self.patch_textures.is_none() || self.original_texture.as_ref() != Some(texture_handle) {
            let mut patch_textures = vec![];
            let mut accu_y = 0;
            for row in &self.patches {
                let mut accu_x = 0;
                for column_item in row {
                    let start_x = accu_x;
                    let end_x = accu_x + to_width(column_item.original_size, texture_size);

                    let start_y = accu_y;
                    let end_y = accu_y + to_height(column_item.original_size, texture_size);

                    let mut patch_texture_data = vec![];
                    for j in start_y as usize..end_y as usize {
                        let start_line = (start_x as usize + j * texture_size.width as usize) * 4;
                        let end_line = (end_x as usize + j * texture_size.width as usize) * 4;
                        patch_texture_data.extend_from_slice(&texture_data[start_line..end_line]);
                    }

                    let patch_texture = Image::new(
                        Extent3d {
                            width: to_width(column_item.original_size, texture_size),
                            height: to_height(column_item.original_size, texture_size),
                            depth_or_array_layers: texture_size.depth_or_array_layers,
                        },
                        TextureDimension::D2,
                        patch_texture_data,
                        TextureFormat::Rgba8UnormSrgb,
                    );
                    textures_to_add.push(patch_texture);
                    accu_x += to_width(column_item.original_size, texture_size);
                }
                accu_y += to_height(row[0].original_size, texture_size);
            }
            textures_to_add.into_iter().for_each(|patch_texture| {
                let patch_texture_handle = textures.add(patch_texture);
                patch_textures.push(patch_texture_handle);
            });
            self.patch_textures = Some(patch_textures);
            self.original_texture = Some(texture_handle.clone());
        }
        NinePatch {
            patches: self.patches.clone(),
            texture_size,
            // background: materials.add(Color::NONE.into()),
            splitted_texture: self.patch_textures.as_ref().cloned().unwrap(),
        }
    }
}

/// Component to mark the entity placed for the content of the 9-Patch UI element
#[derive(Clone, Debug, Component)]
pub struct NinePatchContent<T: Send + Sync + 'static> {
    /// Name of the content patch
    pub content: T,
    /// Has it been already loaded
    pub loaded: bool,
    /// Entity parent of the 9-Patch UI element
    pub parent: Entity,
}

/// `NinePatch` ready to be added to entities.
#[derive(Debug)]
pub struct NinePatch<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> {
    patches: Vec<Vec<Patch<T>>>,
    texture_size: Extent3d,
    // background: Handle<Image>,
    splitted_texture: Vec<Handle<Image>>,
}
impl<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> NinePatch<T> {
    pub(crate) fn add_with_parent(
        &self,
        commands: &mut Commands,
        parent: Entity,
        style: &Style,
        contents: &Option<std::collections::HashMap<T, Entity>>,
    ) {
        commands.entity(parent).insert(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Stretch,
                ..style.clone()
            },
            background_color: BackgroundColor(Color::NONE),
            focus_policy: FocusPolicy::Pass,
            ..default()
        });
        let mut rows = vec![];
        let mut n = 0;
        for row in self.patches.iter() {
            let (size_height, growth) =
                row.first()
                    .map_or((Val::ZERO, 0.), |p| match p.target_height {
                        Val::Px(i) => {
                            let height = if i == 0. {
                                Val::Px(to_height(p.original_size, self.texture_size) as f32)
                            } else {
                                Val::Px(i)
                            };
                            (height, 0.)
                        }
                        Val::Auto => (Val::Auto, 1.),
                        Val::Percent(x) => (Val::Auto, x / 100.),
                        other => (other, 0.),
                    });

            let id = commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: size_height,
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::Stretch,
                        flex_grow: growth,
                        flex_shrink: growth,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .id();
            rows.push(id);
            commands.entity(id).with_children(|row_parent| {
                for column_item in row.iter() {
                    let (size_width, growth) = match column_item.target_width {
                        Val::Px(i) => {
                            let width = if i == 0. {
                                Val::Px(
                                    to_width(column_item.original_size, self.texture_size) as f32
                                )
                            } else {
                                Val::Px(i)
                            };
                            (width, 0.)
                        }
                        Val::Auto => (Val::Auto, 1.),
                        Val::Percent(x) => (Val::Auto, x / 100.),
                        _ => todo!(),
                    };
                    let size_height = match column_item.target_height {
                        Val::Px(i) => {
                            if i == 0. {
                                Val::Px(
                                    to_height(column_item.original_size, self.texture_size) as f32
                                )
                            } else {
                                Val::Px(i)
                            }
                        }
                        Val::Percent(_) => Val::Auto,
                        other => other,
                    };
                    let mut child = row_parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: self.splitted_texture[n].clone_weak(),
                            ..default()
                        },
                        style: Style {
                            width: size_width,
                            height: size_height,
                            flex_grow: growth,
                            flex_shrink: growth,
                            ..default()
                        },
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    });
                    if let Some(content_part) = column_item.content.as_ref() {
                        child.insert(NinePatchContent {
                            content: content_part.clone(),
                            loaded: false,
                            parent,
                        });
                        if let Some(content_entity) =
                            contents.as_ref().and_then(|m| m.get(content_part))
                        {
                            child.push_children(&[*content_entity]);
                        }
                    }
                    n += 1;
                }
            });
        }
        commands.entity(parent).push_children(&rows);
    }
}
