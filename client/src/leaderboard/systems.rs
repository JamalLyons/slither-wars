use bevy::prelude::*;
use crate::player::components::Player;
use crate::bot::components::Bot;
use super::components::*;
use crate::constants::*;

pub fn spawn_leaderboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the leaderboard container
    commands.spawn((
        Leaderboard,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(BLACK_COLOR),
            ..default()
        },
    ))
    .with_children(|parent| {
        // Leaderboard title
        parent.spawn(
            TextBundle::from_section(
                "Leaderboard",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }),
        );
    });
}

pub fn update_leaderboard(
    mut commands: Commands,
    leaderboard_query: Query<Entity, With<Leaderboard>>,
    player_query: Query<(&Player, &Name)>,
    bot_query: Query<&Bot>,
    asset_server: Res<AssetServer>,
) {
    // Get all scores and names
    let mut scores: Vec<(String, u32)> = Vec::new();

    // Add player scores
    for (player, name) in player_query.iter() {
        scores.push((name.to_string(), player.score));
    }

    // Add bot scores
    for (i, bot) in bot_query.iter().enumerate() {
        scores.push((format!("Bot {}", i + 1), bot.score));
    }

    // Sort scores in descending order
    scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Update the leaderboard UI
    if let Ok(leaderboard_entity) = leaderboard_query.get_single() {
        commands.entity(leaderboard_entity).despawn_descendants();
        
        // Spawn title
        commands.entity(leaderboard_entity).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Leaderboard",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                }),
            );

            // Spawn entries
            for (i, (name, score)) in scores.iter().take(10).enumerate() {
                parent.spawn(
                    TextBundle::from_section(
                        format!("{}. {} - {}", i + 1, name, score),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            color: if i == 0 { LEADERBOARD_COLOR } else { LEADERBOARD_ENTRY_COLOR },
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    }),
                );
            }
        });
    }
} 