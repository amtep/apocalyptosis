use bevy::prelude::*;
use strum::{EnumIter, IntoStaticStr};

use crate::{
    constants::STARTING_FUNDS,
    state::{GameState, MainSetupSet},
    time::GameDateChangedEvent,
};

pub struct FundsPlugin;

impl Plugin for FundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Main),
            setup_funds.in_set(MainSetupSet::Default),
        );
    }
}

pub type FundsAmount = i64;

#[derive(Resource)]
pub struct Funds(pub FundsAmount);

#[derive(Event)]
pub struct FundsChangedEvent;

#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct Expense(pub FundsAmount, pub ExpenseCategory);

#[allow(dead_code)] // TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum ExpenseCategory {
    Followers,
    Bases,
}

#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct Income(pub FundsAmount, pub IncomeCategory);

#[allow(dead_code)] // TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum IncomeCategory {
    Jobs,
    Crime,
}

fn setup_funds(mut commands: Commands) {
    commands.insert_resource(Funds(STARTING_FUNDS));
    commands.add_observer(update_funds);
}

fn update_funds(
    _: On<GameDateChangedEvent>,
    mut commands: Commands,
    mut funds: ResMut<Funds>,
    incomes: Query<&Income>,
    expenses: Query<&Expense>,
) {
    for Income(amount, _) in incomes {
        funds.0 += amount;
    }
    for Expense(amount, _) in expenses {
        funds.0 -= amount;
    }
    commands.trigger(FundsChangedEvent);
}
