use bevy::prelude::*;
use chrono::{Days, NaiveDate};

#[derive(Resource)]
pub struct GameDate(pub NaiveDate);

impl Default for GameDate {
    fn default() -> Self {
        Self(NaiveDate::from_ymd_opt(2026, 4, 15).unwrap())
    }
}

pub fn setup_game_time(mut commands: Commands) {
    commands.insert_resource(Time::<Fixed>::from_seconds(1.0));
    commands.insert_resource(GameDate::default());
    // TODO: can't update game time UI yet because texts have not yet loaded.
}

#[derive(Event)]
pub struct GameDateChanged;

pub fn advance_game_time(mut commands: Commands, mut date: ResMut<GameDate>) {
    // We don't expect to reach 262000 AD
    date.0 = date.0 + Days::new(1);
    commands.trigger(GameDateChanged);
}
