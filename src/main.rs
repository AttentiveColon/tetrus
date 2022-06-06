use macroquad::rand::gen_range;

use macroquad::prelude::*;
mod icon;
use icon::ICON;

mod tetrus;
use tetrus::Tetrus;

mod constants;

fn get_mq_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: String::from("Tetrus"),
        window_height: 50, //1000,
        window_width: 50,  //600,
        fullscreen: false,
        window_resizable: false,
        icon: Some(ICON),
        ..Default::default()
    }
}

fn is_running() -> bool {
    if is_key_pressed(KeyCode::Escape) {
        false
    } else {
        true
    }
}

// fn step(time_step: f64, tetrus: &mut Tetrus) {
//     //sleep(time_step);
//     tetrus.check_collision();
//     tetrus.move_unlocked();
//     println!("{:?}", tetrus);
// }

#[macroquad::main(get_mq_conf)]
async fn main() {
    let mut last_update = get_time();
    let tick = 0.5;
    let mut tetrus = Tetrus::new();

    while is_running() {

        
        //testing
        if is_key_pressed(KeyCode::O) {
            let num = gen_range(0, 7);
            match num {
                0 => tetrus.spawn_i(),
                1 => tetrus.spawn_j(),
                2 => tetrus.spawn_l(),
                3 => tetrus.spawn_o(),
                4 => tetrus.spawn_s(),
                5 => tetrus.spawn_t(),
                6 => tetrus.spawn_z(),
                _ => panic!(),
            }
        }
        if get_time() - last_update > tick {
            last_update = get_time();
            tetrus.check_collision();
            tetrus.move_unlocked();
            println!("{:?}", tetrus);
        }



        //step(frame_t, &mut tetrus);
        if tetrus.is_game_over() {
            println!("GAME OVER");
            break;
        }
        next_frame().await
    }
}
