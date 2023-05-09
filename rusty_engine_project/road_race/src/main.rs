use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

struct GameState {
    health_amount: u8,
    lost: bool,
}

// GameState::default()
impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 3,
            lost: false,
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);

    player1.translation.x = -500.0;
    player1.translation.y = -100.0;
    player1.layer = 10.0;
    player1.collision = true;

    let player2 = game.add_sprite("player2", SpritePreset::RacingCarBlack);
    player2.translation.x = -500.0;
    player2.translation.y = 100.0;
    player2.layer = 10.0;
    player2.collision = true;

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // HP 상태 메세지
    let health_message = game.add_text("health_message", "Health: 3");
    health_message.translation = Vec2::new(550.0, 320.0);

    // 도로 그리기
    for i in 0..10 {
        let road_line =
            game.add_sprite(format!("road_line{}", i), SpritePreset::RacingBarrierWhite);
        road_line.scale = 0.1;
        road_line.translation.x = -600.0 + 150.0 * i as f32;
    }

    // 장애물
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.health_amount == 0 {
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine
            .audio_manager
            .play_sfx(SfxPreset::Confirmation1, 0.03);
    }

    let mut p1_direction = 0.0;
    let mut p2_direction = 0.0;

    if engine.keyboard_state.pressed(KeyCode::Up) {
        p1_direction += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        p1_direction -= 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::W) {
        p2_direction += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::S) {
        p2_direction -= 1.0;
    }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += p1_direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = p1_direction * 0.15;

    let player2 = engine.sprites.get_mut("player2").unwrap();
    player2.translation.y += p2_direction * PLAYER_SPEED * engine.delta_f32;
    player2.rotation = p2_direction * 0.15;

    for sprite in engine.sprites.values_mut() {
        // 도로 움직이기
        if sprite.label.starts_with("road_line") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
        // 장애물 움직이기
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        // 이벤트의 pair에서 두개 중 하나도 player1이 아니거나 이벤트가 끝이 라면 continue
        if !(event.pair.either_contains("player1") || event.pair.either_contains("player2"))
            || event.state.is_end()
        {
            continue;
        }

        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }
}
