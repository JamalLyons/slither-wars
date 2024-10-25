use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::shared::*;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Game), game_setup);
        app.add_systems(Update, (move_player, update_camera).chain().run_if(in_state(GameState::Game)));
        // Cleans up game entities. This is useful so when the player starts a new game, the state is reset.
        app.add_systems(
            OnExit(GameState::Game),
            (despawn_screen::<Player>, despawn_screen::<Segment>, despawn_screen::<Orb>),
        );
    }
}

pub fn game_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    // Spawn the Player entity
    let player_entity = commands
        .spawn((
            Player::new("Player".to_string()),
            Transform::default(),
            GlobalTransform::default(),
            PositionHistory::default(),
        ))
        .id();

    // Spawn segments and attach them to the player
    for i in 0..PLAYER_DEFAULT_LENGTH {
        let segment_entity = commands
            .spawn((
                Segment,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(12.5)).into(),
                    material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                    transform: Transform {
                        translation: Vec3::new(-(i as f32) * SEGMENT_SPACING, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        // Add the segment to the player's segments vector
        commands.entity(player_entity).push_children(&[segment_entity]);
    }
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Transform, &mut PositionHistory), With<Player>>,
    mut segment_query: Query<&mut Transform, (With<Segment>, Without<Player>)>,
)
{
    // End game if escape is pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }

    for (mut player_transform, mut history) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }

        // Record player's position
        history.positions.push_front(player_transform.translation);
        if history.positions.len() > MAX_SEGMENT_HISTORY {
            history.positions.pop_back();
        }

        // Update segments
        for (i, mut segment_transform) in segment_query.iter_mut().enumerate() {
            let index = (i + 1) * POSITIONS_PER_SEGMENT;
            if index < history.positions.len() {
                segment_transform.translation = history.positions[index];
            } else {
                // If not enough history, set position to player's current position
                segment_transform.translation = player_transform.translation;
            }
        }
    }
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
)
{
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    let Ok(player) = player_query.get_single() else {
        return;
    };

    let target = Vec3::new(player.translation.x, player.translation.y, camera.translation.z);

    camera.translation = camera.translation.lerp(target, time.delta_seconds() * CAM_LERP_FACTOR);
}
