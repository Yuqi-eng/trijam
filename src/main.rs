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
    let background = load_texture("GAME_ROOM_2.png").await.unwrap();

    loop {
        clear_background(WHITE);
        draw_texture(background, 0., 0., WHITE);
        draw_circle(screen_width(), screen_height(), 15.0, YELLOW);

        next_frame().await
    }
}

