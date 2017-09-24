extern crate ggez;
extern crate rand;

use rand::Rng;

use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Point};
use ggez::timer;
use std::time::Duration;

// smashable x1
// smashable x2
// window w
// window h
// smashable spawn room
// smashable amount
// smashable collision area
// player x
// hitarea
// player walking speed
// player holding speed
// penalty time
// hold up time min
// hold up time max
// player warp y
// player warp state max
// player warp state intermediate

struct Smashable {
  x: f32,
  y: f32,
  t: i32,
  active: bool,
  car: graphics::Image,
  cctv: graphics::Image,
  hydrant: graphics::Image
}

impl Smashable {
  fn new(ctx: &mut Context) -> Smashable {
    let mut rng = rand::thread_rng();
    let y = rng.gen::<f32>() * 550.0 + 100.0; // magic number
    let x:f32;
    let ltr = rng.gen();
    match ltr {
      true => { x = 135.0 } // magic number
      false => { x = 255.0 } //magic number
    }
    let t = rand::thread_rng().gen_range(1, 4);

    let car = graphics::Image::new(ctx, "/car-sprite.png").unwrap();
    let cctv = graphics::Image::new(ctx, "/cctv-sprite.png").unwrap();
    let hydrant = graphics::Image::new(ctx, "/hydrant-sprite.png").unwrap();

    Smashable {
      x: x,
      y: y,
      t: t,
      active: true,
      car: car,
      cctv: cctv,
      hydrant: hydrant
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let point = graphics::Point::new(self.x, self.y);

    if self.active {
      match self.t {
        2 => {
          graphics::draw(ctx, &self.car, point, 0.0)?;
        }
        3 => {
          graphics::draw(ctx, &self.cctv, point, 0.0)?;
        }
        _ => {
          graphics::draw(ctx, &self.hydrant, point, 0.0)?;
        }
      }
    }
    Ok(())
  }
}

struct Player {
  x: f32,
  y: f32,
  sprite1: graphics::Image,
  sprite2: graphics::Image,
  sprite3: graphics::Image,
  hitarea: graphics::Image,
  h_x: f32,
  h_y: f32,
  h_w: f32,
  h_h: f32,
  holding: f32,
  penalty: f32
}

impl Player {
  fn new(ctx: &mut Context) -> Player {
    Player {
      x: 195.0,
      y: 20.0,
      sprite1: graphics::Image::new(ctx, "/beyonce.png").unwrap(),
      sprite2: graphics::Image::new(ctx, "/beyonce-bat.png").unwrap(),
      sprite3: graphics::Image::new(ctx, "/beyonce-swing.png").unwrap(),
      hitarea: graphics::Image::new(ctx, "/swing.png").unwrap(),
      h_x: 195.0,
      h_y: 195.0 + (64.0 / 2.0),
      h_w: 128.0,
      h_h: 32.0,
      holding: 0.0,
      penalty: 0.0
    }
  }

  pub fn update(&mut self) {
    if self.penalty > 0.0 {
      self.penalty += 0.1;
      if self.penalty > 8.0 { // magic
        self.penalty = 0.0;
      }
    }
    if self.holding == 0.0 {
      self.y = self.y % 700.0 + 2.0; // magic
      self.h_y = self.y + (64.0 / 2.0);
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dest_point = graphics::Point::new(self.x, self.y);

    if self.holding > 4.0 { // magic number
      let dest_hitarea = graphics::Point::new(self.h_x, self.h_y);
      graphics::draw(ctx, &self.sprite3, dest_point, 0.0)?;
      graphics::draw(ctx, &self.hitarea, dest_hitarea, 0.0)?;
    } else if self.holding > 0.0 {
      graphics::draw(ctx, &self.sprite2, dest_point, 0.0)?;
    } else {
      graphics::draw(ctx, &self.sprite1, dest_point, 0.0)?;
    }

    Ok(())
  }

  pub fn hold(&mut self) {
    if self.penalty == 0.0 {
      if self.holding > 0.0 {
        self.holding += 0.3; // magic

        if self.holding > 6.0 { // magic number
          self.penalty = 0.1;
          self.unhold();
        }
      } else {
        self.holding = 0.1;
      }
    }
  }

  pub fn unhold(&mut self) {
    self.holding = 0.0;
  }
}

struct MainState {
  player: Player,
  font: graphics::Font,
  title: graphics::Text,
  holdup: graphics::Text,
  street: graphics::Image,
  smashables: Vec<Smashable>,
  score: u32,
  time: u32,
  state: u32
}

impl MainState {
  fn new(ctx: &mut Context) -> GameResult<MainState> {
    let font = graphics::Font::new(ctx, "/leaguespartan-bold.ttf", 30)?;
    let title = graphics::Text::new(ctx, "Beyoncé Brawles", &font)?;
    let holdup = graphics::Text::new(ctx, "HOLD UP!", &font)?;
    let street = graphics::Image::new(ctx, "/street-2.png")?;

    let mut smashables = vec![];

    for _ in 0..13 { // magic
      smashables.push(Smashable::new(ctx));
    }

    let s = MainState {
      player: Player::new(ctx),
      font: font,
      title: title,
      holdup: holdup,
      street: street,
      smashables: smashables,
      score: 0,
      time: 0,
      state: 0
    };
    Ok(s)
  }

  pub fn collision(&mut self) {
    if self.player.holding > 4.0 { //magic number
      for s in self.smashables.iter_mut() {
        if s.active {
          if self.player.h_x < s.x + 64.0 && // magic
            self.player.h_x + self.player.h_w > s.x &&
            self.player.h_y < s.y + 64.0 && //magic
            self.player.h_y + self.player.h_h > s.y {
              s.active = false;
              self.score += 1; // magic
            }
        }
      }
    }
  }

  pub fn respawn(&mut self, ctx: &mut Context) {
    self.smashables = vec![];
    for _ in 0..13 { // magic
      self.smashables.push(Smashable::new(ctx));
    }
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
    if self.state < 3 {
      self.player.update();
      self.time = (timer::duration_to_f64(timer::get_time_since_start(_ctx)) * 1000.0) as u32 / 1000;
    }

    if self.player.y == 698.0 { // magic
      self.state += 1;
      self.respawn(_ctx);
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    graphics::draw(ctx, &self.street, Point { x: self.street.width() as f32 / 2.0, y: self.street.height() as f32 / 2.0 }, 0.0)?;

    if self.state == 0 || self.state > 3 { // magic
      graphics::draw(ctx, &self.title, Point { x: 200.0, y: self.title.height() as f32 }, 0.0)?;
    }

    if self.state < 3 { // magic
      let time = graphics::Text::new(ctx, &self.time.to_string(), &self.font)?;
      graphics::draw(ctx, &time, Point { x: 360.0, y: 670.0 }, 0.0)?;

      let score = graphics::Text::new(ctx, &self.score.to_string(), &self.font)?;
      graphics::draw(ctx, &score, Point { x: 40.0, y: 670.0 }, 0.0)?;

      if self.player.penalty > 0.0 {
        let penalty_txt = graphics::Text::new(ctx, "X", &self.font).unwrap();
        graphics::draw(ctx, &penalty_txt, Point { x: self.player.x, y: self.player.y - 64.0 }, 0.0)?;
      }

      for s in self.smashables.iter_mut() {
        s.draw(ctx)?;
      }

      if self.player.holding >= 1.0 && self.player.holding < 4.0 {
        let holdhelp = self.player.holding as u32;
        let holdtime = graphics::Text::new(ctx, &holdhelp.to_string(), &self.font).unwrap();
        graphics::draw(ctx, &holdtime, Point { x: self.player.x, y: self.player.y - 64.0 }, 0.0)?;
      }
      if self.player.holding >= 4.0 { // magic
        graphics::draw(ctx, &self.holdup, Point { x: self.player.x, y: self.player.y - 64.0 }, 0.0)?;
      }

      self.player.draw(ctx)?;
    }

    if self.state > 3 {
      graphics::draw(ctx, &self.holdup, Point { x: 200.0, y: 200.0 }, 0.0)?;
      let finalscore = self.score * 100 / (self.time + 1);
      let scorestring = graphics::Text::new(ctx, &finalscore.to_string(), &self.font)?;
      graphics::draw(ctx, &scorestring, Point { x: 200.0, y: 250.0}, 0.0)?;
    }

    graphics::present(ctx);
    Ok(())
  }
  fn key_down_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
    match keycode {
      Keycode::Space => {
        self.player.hold();
      }
      _ => {}
    }
  }
  fn key_up_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
    match keycode {
      Keycode::Space => {
        self.collision();
        self.player.unhold();
      }
      _ => {}
    }
  }
}

pub fn main() {
  let mut c = conf::Conf::new();
  c.window_title = "Beyoncé Brawles".to_string();
  c.window_width = 400;
  c.window_height = 700;
  c.window_icon = "/beyonce-swing.png".to_string();

  let ctx = &mut Context::load_from_conf("beyonce_brawles", "ggez", c).unwrap();
  let state = &mut MainState::new(ctx).unwrap();

  if let Err(e) = event::run(ctx, state) {
    println!("Error encountered: {}", e);
  } else {
    println!("Game exited cleanly.");
  }
}
