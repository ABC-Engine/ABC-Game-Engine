// this highlights some major issues with the current renderer
use ABC_Game_Engine::*;

const WINDOW_DIMS: (u32, u32) = (80, 80);

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new(WINDOW_DIMS.0, WINDOW_DIMS.1);
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    {
        let entities_and_components = &mut scene.game_engine.entities_and_components;

        scene.scene_params.set_background_color(Color {
            r: 255,
            g: 255,
            b: 255,
            a: 1.0,
        });

        scene.scene_params.set_random_chars(false);
        scene.scene_params.set_character('█');

        let animation = load_spritesheet(4, 100, "sprite_sheet_golem_0_16x16.png");

        entities_and_components.add_entity_with((
            Sprite::Animation(animation),
            Transform {
                x: 40.0,
                y: 20.0,
                scale: 1.0,
                rotation: 0.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        ));
    }

    loop {
        scene.game_engine.run();
        // should be implemented as a system later
        renderer.render(
            &mut scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
