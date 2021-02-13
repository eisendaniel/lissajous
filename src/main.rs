use ggez::{
    conf, event, graphics, input::keyboard, nalgebra as na, Context, ContextBuilder, GameResult,
};

const WIN_SIZE: f32 = 1024.0;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(conf::WindowMode::default().dimensions(WIN_SIZE, WIN_SIZE))
        .window_setup(conf::WindowSetup::default().samples(conf::NumSamples::Eight))
        .build()
        .expect("Failed to create context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = LissajousCurves::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited Cleanly"),
        Err(e) => println!("Error: {}", e),
    }
}

struct LissajousCurves {
    x_mod: f32,
    y_mod: f32,
    delta: f32,
}

impl LissajousCurves {
    pub fn new(_ctx: &mut Context) -> LissajousCurves {
        // Load/create resources such as images here.
        LissajousCurves {
            x_mod: 1.0,
            y_mod: 1.0,
            delta: 0.01,
        }
    }
}

impl ggez::event::EventHandler for LissajousCurves {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.delta = 0.01;
        if keyboard::is_mod_active(ctx, event::KeyMods::SHIFT) {
            self.delta = 0.001;
        } else if keyboard::is_mod_active(ctx, event::KeyMods::CTRL) {
            self.delta = 0.1;
        }

        if keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
            self.y_mod += self.delta;
        }
        if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
            self.y_mod -= self.delta;
        }
        if keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
            self.x_mod += self.delta;
        }
        if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
            self.x_mod -= self.delta;
        }
        if keyboard::is_key_pressed(ctx, event::KeyCode::Space) {
            self.x_mod = 1.0;
            self.y_mod = 1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mb = &mut graphics::MeshBuilder::new();
        let mut points = Vec::new();
        let mut n: f32 = 0.0;
        while n <= std::f32::consts::TAU {
            points.push(na::Point2::new(
                (0.9 * (self.x_mod * n).sin() + 1.0) * WIN_SIZE / 2.0,
                (0.9 * (self.y_mod * n).cos() + 1.0) * WIN_SIZE / 2.0,
            ));
            n += std::f32::consts::TAU / 2048.;
        }

        mb.polyline(
            graphics::DrawMode::stroke(1.),
            &points,
            [0.5, 1., 0.5, 1.].into(),
        )?;
        mb.polyline(
            graphics::DrawMode::stroke(8.),
            &points,
            [0., 1., 0., 0.1].into(),
        )?;
        mb.polyline(
            graphics::DrawMode::stroke(16.),
            &points,
            [0., 1., 0., 0.01].into(),
        )?;
        let m = mb.build(ctx)?;
        graphics::draw(ctx, &m, graphics::DrawParam::new())?;
        graphics::present(ctx)
    }
}
