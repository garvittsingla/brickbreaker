use rusty_engine::prelude::{bevy::math::vec2, *};


#[derive(Resource)]
struct  GameState {
    carposition : Vec2,
}

fn main() {
    let mut game  = Game::new();
    let initial_game_state = GameState { carposition : vec2(0.0, 100.0) };

    let sprite = game.add_sprite("car","car_blue.png");
    sprite.scale = 2.0;
    game.add_logic(game_logic);
    game.run(initial_game_state);
}
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let car = engine.sprites.get_mut("car").unwrap();
     
    car.translation = game_state.carposition;
    println!("car position: {:?}", car.translation);


    if engine.keyboard_state.pressed(KeyCode::ArrowRight){
         car.translation.x += 1.0;
         game_state.carposition = car.translation;
    }

    if engine.keyboard_state.pressed(KeyCode::ArrowLeft){
     
        car.translation.x -= 1.0;
         game_state.carposition = car.translation;
    }

    if engine.keyboard_state.pressed(KeyCode::ArrowUp){
     
        car.translation.y += 1.0;
         game_state.carposition = car.translation;
    }
    if engine.keyboard_state.pressed(KeyCode::ArrowDown){
     
        car.translation.y -= 1.0;
         game_state.carposition = car.translation;
    }

    if engine.keyboard_state.pressed(KeyCode::ShiftRight){
        car.rotation += 1.0;
    }

    
     
}
