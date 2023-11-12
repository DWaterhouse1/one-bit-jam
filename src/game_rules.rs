use bevy::prelude::*;

use crate::config::GAME_RULES;

pub struct GameRulesPlugin;

impl Plugin for GameRulesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_game)
            .add_systems(Update, handle_coin_loss)
            .add_event::<LoseCoinsEvent>();
    }
}

pub struct GameStatePlugin;

#[derive(Event)]
pub struct LoseCoinsEvent(i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct CoinState {
    pub coins_total: i32,
}

fn init_game(mut commands: Commands) {
    commands.spawn(CoinState {
        coins_total: GAME_RULES.starting_coins,
    });
}

fn handle_coin_loss(
    mut q_coins: Query<&mut CoinState>,
    mut ev_loss: EventReader<LoseCoinsEvent>,
) {
    for ev in ev_loss.iter() {
        let mut coins = match q_coins.get_single_mut() {
            Ok(coins) => coins,
            Err(_) => return,
        } ;

        let loss = ev.0;

        if loss > coins.coins_total {
            // TODO send game over event
            return;
        }

        coins.coins_total -= loss;
    }
}