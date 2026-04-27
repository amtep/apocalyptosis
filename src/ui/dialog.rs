use bevy::{
    ecs::system::IntoObserverSystem,
    prelude::*,
    ui::{FocusPolicy, InteractionDisabled},
};

use crate::{constants::ui::*, text::TextKey};

#[derive(Component)]
pub struct DialogRoot;

#[derive(Debug)]
pub enum DialogBody {
    Text(TextKey),
    Entity(Entity),
}

#[derive(Debug, Default)]
pub struct DialogBuilder {
    font: Handle<Font>,
    text_body_font: Option<Handle<Font>>,
    pause: bool,
    title: Option<TextKey>,
    body: Option<DialogBody>,
    /// default label: "Confirm"
    confirm_label: Option<TextKey>,
    confirm_disabled: bool,
    /// default label: None (no cancel button)
    cancel_label: Option<Option<TextKey>>,
}

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct DialogConfirmEvent {
    pub entity: Entity,
    pub enabled: bool,
}

#[derive(Component)]
struct ConfirmButton(Entity);

pub fn dialog_default_action(_: On<Pointer<Click>>) {}

impl DialogBuilder {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            font,
            ..Default::default()
        }
    }
}

impl DialogBuilder {
    pub fn with_pause(self) -> Self {
        Self {
            pause: true,
            ..self
        }
    }

    pub fn with_confirm_disabled(self) -> Self {
        Self {
            confirm_disabled: true,
            ..self
        }
    }

    pub fn with_title(self, title: impl Into<TextKey>) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn with_text_body(self, text: impl Into<TextKey>) -> Self {
        Self {
            body: Some(DialogBody::Text(text.into())),
            ..self
        }
    }

    pub fn with_text_body_font(self, font: Handle<Font>) -> Self {
        Self {
            text_body_font: Some(font),
            ..self
        }
    }

    pub fn with_entity_body(self, entity: Entity) -> Self {
        Self {
            body: Some(DialogBody::Entity(entity)),
            ..self
        }
    }

    pub fn with_confirm_label(self, label: impl Into<TextKey>) -> Self {
        Self {
            confirm_label: Some(label.into()),
            ..self
        }
    }

    pub fn with_cancel(self) -> Self {
        Self {
            cancel_label: Some(None),
            ..self
        }
    }

    pub fn with_cancel_label(self, label: impl Into<TextKey>) -> Self {
        Self {
            cancel_label: Some(Some(label.into())),
            ..self
        }
    }

    pub fn build<B, M, O>(self, mut commands: Commands, confirm_action: O)
    where
        O: IntoObserverSystem<Pointer<Click>, B, M>,
        B: Bundle,
    {
        let dialog_background = commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                FocusPolicy::Block,
            ))
            .id();

        let mut entity_commands = commands.spawn((
            ChildOf(dialog_background),
            DialogRoot,
            Node {
                left: percent(50),
                top: percent(50),
                min_width: percent(50),
                max_width: percent(50),
                min_height: percent(50),
                max_height: percent(75),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(10)),
                padding: UiRect::axes(px(20), px(5)),
                ..Default::default()
            },
            UiTransform {
                translation: Val2::percent(-50.0, -50.0),
                ..Default::default()
            },
            BorderColor::all(BORDER_HIGHLIGHT),
            BackgroundColor::from(DIALOG_BACKGROUND),
            ZIndex(1),
        ));

        let dialog_root = entity_commands.id();

        let hrule = (
            Node {
                width: percent(90),
                height: px(1),
                margin: UiRect::vertical(px(5)),
                ..default()
            },
            BackgroundColor::from(BORDER),
        );

        if let Some(title) = self.title {
            entity_commands
                .with_child((
                    title,
                    TextColor::from(TEXT),
                    TextFont::from_font_size(HEADING).with_font(self.font.clone()),
                ))
                .with_child(hrule.clone());
        }

        if let Some(body) = self.body {
            match body {
                DialogBody::Text(text_key) => {
                    let font = self.text_body_font.unwrap_or_else(|| self.font.clone());
                    entity_commands.with_child((
                        text_key,
                        TextColor::from(TEXT),
                        TextLayout::new_with_justify(Justify::Justified),
                        TextFont::from_font_size(SMALL).with_font(font),
                    ))
                }
                DialogBody::Entity(entity) => {
                    entity_commands.observe(
                        |mut dialog_confirm: On<DialogConfirmEvent>,
                         mut commands: Commands,
                         confirm_buttons: Query<&ConfirmButton>| {
                            if let Ok(confirm_button) = confirm_buttons.get(dialog_confirm.entity) {
                                if dialog_confirm.enabled {
                                    commands
                                        .entity(confirm_button.0)
                                        .try_remove::<InteractionDisabled>();
                                } else {
                                    commands
                                        .entity(confirm_button.0)
                                        .insert(InteractionDisabled);
                                }
                            }
                            dialog_confirm.propagate(false);
                        },
                    );
                    entity_commands.add_child(entity)
                }
            }
            .with_child(Node {
                flex_grow: 1.0,
                ..default()
            })
            .with_child(hrule);
        }

        entity_commands.with_children(|parent| {
            parent
                .spawn(Node {
                    width: percent(90),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::vertical(px(5)),
                    column_gap: percent(5),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let button = |text_key| {
                        (
                            Node {
                                width: percent(50),
                                height: px(40),
                                border: UiRect::all(px(1)),
                                border_radius: BorderRadius::all(px(5)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderColor::all(BORDER),
                            BackgroundColor::from(MENU_BACKGROUND),
                            Button,
                            children![(
                                text_key,
                                TextColor::from(TEXT),
                                TextFont::from_font_size(LARGE).with_font(self.font.clone()),
                            )],
                        )
                    };

                    if let Some(cancel_label) = self.cancel_label {
                        let cancel_label =
                            cancel_label.unwrap_or_else(|| TextKey::new("dialog-cancel"));

                        parent.spawn(button(cancel_label)).observe(
                            move |_: On<Pointer<Click>>, mut commands: Commands| {
                                commands.entity(dialog_background).despawn();
                            },
                        );
                    }

                    let confirm_label = self
                        .confirm_label
                        .unwrap_or_else(|| TextKey::new("dialog-confirm"));

                    let mut confirm_button = parent
                        .spawn(button(confirm_label));

                    if self.confirm_disabled {
                        confirm_button.insert(InteractionDisabled);
                    }

                    confirm_button
                        .observe(move |click: On<Pointer<Click>>, mut commands: Commands, has_disableds: Query<Has<InteractionDisabled>>| {
                            if !has_disableds.get(click.entity).unwrap() {
                                commands.entity(dialog_background).despawn();
                            }
                        })
                        .observe(confirm_action);

                    let confirm_button = confirm_button.id();
                    parent.commands().entity(dialog_root).insert(ConfirmButton(confirm_button));
                });
        });
    }
}
