use bevy::prelude::*;

use crate::{constants::STARTING_FUNDS, time::GameDateChanged};

#[derive(Resource)]
pub struct Funds(pub i64);

#[derive(Event)]
pub struct FundsChanged;

#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct Expense(pub i64, pub ExpenseCategory);

#[allow(dead_code)] // TODO
pub enum ExpenseCategory {
    Followers,
    Bases,
}

#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct Income(pub i64, pub IncomeCategory);

#[allow(dead_code)] // TODO
pub enum IncomeCategory {
    Jobs,
    Crime,
}

pub fn setup_funds(mut commands: Commands) {
    commands.insert_resource(Funds(STARTING_FUNDS));
    commands.add_observer(update_funds);
}

pub fn update_funds(
    _: On<GameDateChanged>,
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
    commands.trigger(FundsChanged);
}
