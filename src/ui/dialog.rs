use bevy::prelude::*;

use crate::{constants::ui::*, text::TextKey};

#[derive(Component)]
pub struct DialogRoot;

#[derive(Debug)]
pub enum DialogBody {
    Text(TextKey),
    Entity(Entity),
}

#[derive(Debug)]
pub struct DialogBuilder<F1 = fn(&mut Commands), F2 = fn(&mut Commands)>
where
    F1: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
    F2: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
{
    font: Handle<Font>,
    text_body_font: Option<Handle<Font>>,
    pause: bool,
    title: Option<TextKey>,
    body: Option<DialogBody>,
    /// default label: "Confirm"
    confirm_label: Option<TextKey>,
    confirm_action: Option<F1>,
    /// default label: None (no cancel button)
    cancel_label: Option<Option<TextKey>>,
    cancel_action: Option<F2>,
}

impl Default for DialogBuilder {
    fn default() -> Self {
        Self {
            font: Default::default(),
            text_body_font: Default::default(),
            pause: Default::default(),
            title: Default::default(),
            body: Default::default(),
            confirm_label: Default::default(),
            confirm_action: Default::default(),
            cancel_label: Default::default(),
            cancel_action: Default::default(),
        }
    }
}

impl DialogBuilder {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            font,
            ..Default::default()
        }
    }
}

impl<F1, F2> DialogBuilder<F1, F2>
where
    F1: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
    F2: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
{
    pub fn with_pause(self) -> Self {
        Self {
            pause: true,
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

    pub fn build(mut self, commands: &mut Commands) {
        let mut entity_commands = commands.spawn((
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
                DialogBody::Entity(entity) => entity_commands.add_child(entity),
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
                        let cancel_label = cancel_label.unwrap_or_else(|| TextKey::new("cancel"));

                        parent.spawn(button(cancel_label)).observe(
                            move |_: On<Pointer<Click>>, mut commands: Commands| {
                                if let Some(cancel_action) = self.cancel_action.take() {
                                    cancel_action(&mut commands);
                                }
                                commands.entity(dialog_root).despawn();
                            },
                        );
                    }

                    let confirm_label = self
                        .confirm_label
                        .unwrap_or_else(|| TextKey::new("confirm"));
                    parent.spawn(button(confirm_label)).observe(
                        move |_: On<Pointer<Click>>, mut commands: Commands| {
                            if let Some(confirm_action) = self.confirm_action.take() {
                                confirm_action(&mut commands);
                            }
                            commands.entity(dialog_root).despawn();
                        },
                    );
                });
        });
    }
}

impl<F1, F2> DialogBuilder<F1, F2>
where
    F1: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
    F2: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
{
    pub fn with_confirm_action<F>(self, action: F) -> DialogBuilder<F, F2>
    where
        F: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
    {
        DialogBuilder {
            font: self.font,
            text_body_font: self.text_body_font,
            pause: self.pause,
            title: self.title,
            body: self.body,
            confirm_label: self.confirm_label,
            confirm_action: Some(action),
            cancel_label: self.cancel_label,
            cancel_action: self.cancel_action,
        }
    }

    pub fn with_cancel_action<F>(self, action: F) -> DialogBuilder<F1, F>
    where
        F: for<'a> FnOnce(&'a mut Commands) + Send + Sync + 'static,
    {
        DialogBuilder {
            font: self.font,
            text_body_font: self.text_body_font,
            pause: self.pause,
            title: self.title,
            body: self.body,
            confirm_label: self.confirm_label,
            confirm_action: self.confirm_action,
            cancel_label: self.cancel_label,
            cancel_action: Some(action),
        }
    }
}
