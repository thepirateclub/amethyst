extern crate amethyst;
extern crate cgmath;

use cgmath::Vector3;

use amethyst::engine::{Application, State, Trans};
use amethyst::context::Context;
use amethyst::config::Element;
use amethyst::ecs::{World, Entity};

struct Example;

impl State for Example {
    fn handle_events(&mut self, events: &[Entity], ctx: &mut Context, _: &mut World) -> Trans {
        use amethyst::context::event::{EngineEvent, Event, VirtualKeyCode};
        let mut trans = Trans::None;
        let storage = ctx.broadcaster.read::<EngineEvent>();
        for e in events {
            let event = storage.get(*e).unwrap();
            let event = &event.payload;
            match *event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => trans = Trans::Quit,
                Event::Closed => trans = Trans::Quit,
                _ => (),
            }
        }
        trans
    }

    fn on_start(&mut self, ctx: &mut Context, _: &mut World) {
        use amethyst::renderer::pass::{Clear, DrawShaded};
        use amethyst::renderer::{Layer, Camera, Light};

        let (w, h) = ctx.renderer.get_dimensions().unwrap();
        let proj = Camera::perspective(60.0, w as f32 / h as f32, 1.0, 100.0);
        let eye = [0., 5., 0.];
        let target = [0., 0., 0.];
        let up = [0., 0., 1.];
        let view = Camera::look_at(eye, target, up);
        let camera = Camera::new(proj, view);

        ctx.renderer.add_scene("main");
        ctx.renderer.add_camera(camera, "main");

        ctx.asset_manager.create_constant_texture("dark_blue", [0.0, 0.0, 0.01, 1.]);
        ctx.asset_manager.create_constant_texture("green", [0.0, 1.0, 0.0, 1.]);
        ctx.asset_manager.gen_sphere("sphere", 32, 32);

        let translation = Vector3::new(0.0, 0.0, 0.0);
        let transform: [[f32; 4]; 4] = cgmath::Matrix4::from_translation(translation).into();
        let fragment = ctx.asset_manager.get_fragment("sphere", "dark_blue", "green", transform).unwrap();

        ctx.renderer.add_fragment("main", fragment);

        let light = Light {
            color: [1., 1., 1., 1.],
            radius: 1.,
            center: [2., 2., 2.],
            propagation_constant: 0.,
            propagation_linear: 0.,
            propagation_r_square: 1.,
        };

        ctx.renderer.add_light("main", light);

        let layer =
            Layer::new("main",
                        vec![
                            Clear::new([0., 0., 0., 1.]),
                            DrawShaded::new("main", "main"),
                        ]);

        let pipeline = vec![layer];
        ctx.renderer.set_pipeline(pipeline);
    }

    fn update(&mut self, ctx: &mut Context, _: &mut World) -> Trans {
        ctx.renderer.submit();
        Trans::None
    }
}

fn main() {
    use amethyst::context::Config;
    let config = Config::from_file(
        format!("{}/config/window_example_config.yml",
                env!("CARGO_MANIFEST_DIR"))
        ).unwrap();
    let ctx = Context::new(config);
    let mut game = Application::build(Example, ctx).done();
    game.run();
}