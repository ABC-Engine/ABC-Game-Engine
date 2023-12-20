# Console-Renderer
A unique renderer that displays through ASCII characters in the terminal
## Blazingly Fast?
It might be blazingly fast 🚀... tbd
# Getting Started
Note: This is subject to change, these docs could be outdated. Just bug me and I will update them for you.
## Rendering an object
``` rust
// this highlights some major issues with the current renderer
use ABC_Game_Engine::*;

// implement a system
struct SpinSystem {}

impl System for SpinSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        for i in 0..entities_and_components.get_entity_count() {
            let entity = entities_and_components.get_nth_entity(i).unwrap();
            entities_and_components
                .get_component_mut::<Transform>(entity)
                .rotation += 1.0;
        }
    }
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new(320, 160);
    // stretch is in case you can't change the line height of your terminal
    renderer.set_stretch(1.0);
    // make a scene to store our systems and objects
    let mut scene = Scene::new();
    // add the system
    scene.game_engine.add_system(Box::new(SpinSystem {}));
    {
        // how we interact with the entities and components
        // needs to be separate from the scene to avoid borrowing issues
        let mut entities_and_components = &mut scene.game_engine.entities_and_components;

        scene.scene_params.set_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        });

        scene.scene_params.set_random_chars(true);

        let sprite = Image {
            texture: load_texture("Sample_Images/sprite.png"),
        };

        let sprite_object = entities_and_components.add_entity();
        entities_and_components.add_component_to(sprite_object, Sprite::Image(sprite));
        entities_and_components.add_component_to(
            sprite_object,
            Transform {
                x: 20.0,
                y: 20.0,
                rotation: 0.0,
                scale: 2.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        );
    }

    loop {
        //Run all the systems
        scene.game_engine.run();

        // render the scene
        renderer.render(
            &scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
```


