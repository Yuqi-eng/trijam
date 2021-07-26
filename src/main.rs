use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width:640,
        window_height:480,
        ..Default::default()
    }
}

struct Cat {
    x: f32,
    y: f32,
    animation_state: usize,
    animation_time: f32,
    sprite: Texture2D,
}

impl Cat {
    fn new(x: f32, y: f32, sprite: Texture2D) -> Cat {
      Cat{
        x: x,
        y: y,
        animation_state: 0,
        animation_time: 0.,
        sprite: sprite
      }
    }

    fn tick(&mut self, delta: f32) {
        self.animation_time += delta;

        if self.animation_time > 0.5 {
          self.animation_time = 0.;
          self.animation_state += 1;
          if self.animation_state > 4 {
            self.animation_state = 0;
          }
        }
    }

    fn draw(&self) {
        let x = (self.animation_state as f32) * 41.;

        draw_texture_ex(self.sprite, self.x, self.y, WHITE,
          DrawTextureParams {
            source: Some(Rect{x: x, y: 0., w: 38., h: 30.}),
            ..Default::default()
          }
        );
    }
}


#[macroquad::main(window_conf)]
async fn main(){
    let background = load_texture("assets/GAME_ROOM_2.png").await.unwrap();
    let cat_sprite = load_texture("assets/cat.png").await.unwrap();

    let mut cats: Vec<Cat> = vec![
      Cat::new(100., 400., cat_sprite),
      Cat::new(140., 380., cat_sprite),
      Cat::new(190., 390., cat_sprite),
      Cat::new(200., 350., cat_sprite)
    ];

    loop {
        clear_background(WHITE);
        draw_texture(background, 0., 0., WHITE);

        // floor outline
        draw_line(69.,  230.,   0., 310., 1., RED);
        draw_line(69.,  230., 508., 230., 1., RED);
        draw_line(640., 334., 508., 230., 1., RED);

        // tick
        let delta = get_frame_time();
        cats.iter_mut().for_each(|cat| cat.tick(delta));

        // draw
        cats.iter_mut().for_each(|cat| cat.draw());

        next_frame().await
    }
}

