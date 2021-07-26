use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width:640,
        window_height:480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main(){
    let background = load_texture("assets/GAME_ROOM_2.png").await.unwrap();

    loop {
        clear_background(WHITE);
        draw_texture(background, 0., 0., WHITE);
        draw_circle(10., 100., 15.0, YELLOW);
        draw_circle(10., 100., 15.0, RED);

        next_frame().await
    }
}

