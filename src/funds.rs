use bevy::prelude::*;
use strum::{EnumIter, IntoStaticStr};

use crate::{
    constants::STARTING_FUNDS,
    state::{GameState, MainSetupSet},
    time::GameDate,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Main),
        setup_funds.in_set(MainSetupSet::Default),
    )
    .add_systems(
        FixedUpdate,
        update_funds.run_if(resource_exists_and_changed::<GameDate>.and(in_state(GameState::Main))),
    );
}

pub type FundsAmount = i64;

#[derive(Resource)]
pub struct Funds(pub FundsAmount);

#[derive(Component)]
pub struct Expense(pub FundsAmount, pub ExpenseCategory);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum ExpenseCategory {
    Followers,
    Bases,
}

#[derive(Component)]
pub struct Income(pub FundsAmount, pub IncomeCategory);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum IncomeCategory {
    Jobs,
    Crime,
}

fn setup_funds(mut commands: Commands) {
    commands.insert_resource(Funds(STARTING_FUNDS));
}

fn update_funds(mut funds: ResMut<Funds>, incomes: Query<&Income>, expenses: Query<&Expense>) {
    for Income(amount, _) in incomes {
        funds.0 += amount;
    }
    for Expense(amount, _) in expenses {
        funds.0 -= amount;
    }
}
