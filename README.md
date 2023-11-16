# Console-Renderer
A unique renderer that displays through ASCII characters in the terminal
## Blazingly Fast?
It might be blazingly fast 🚀... tbd
# Getting Started
Note: This is subject to change, these docs could be outdated. Just bug me and I will update them for you.
## Making an object
First make a struct for your object
``` rust
struct Ball {
    transform: Transform,
    sprite: Sprite,
}
```
Every object needs the Object trait implemented
``` rust
impl Object for Ball {
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}
```
Every object also needs the update function (this will be called every frame)
For this example, we aren't going to use it so we will keep it empty
``` rust
impl Update for Ball {
    fn update(&mut self) {}
}
```
## Instantiating the object
first, let's make the sprite for our object
``` rust
let circle_sprite = Circle {
        radius: 5.0,
        color: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 1.0,
        },
    };
```
now we instantiate the struct we made for our object
``` rust
let circle_object = Ball {
    transform: Transform {
        x: 10.0,
        y: 20.0,
        rotation: 0.0,
        scale: 1.0,
    },
    sprite: circle_sprite.into(),
};
```
Let's make a scene to put our object into
``` rust
let mut main_scene = Scene::new();
```
And a renderer to render it in
``` rust
let mut renderer = Renderer::new(80, 40);
```
now we can add the object to our scene
Note: The order in which they are added is the order they will be rendered in
``` rust
main_scene.add_object(circle_object);
```
Finally, we can now render to our heart's content 
Note: every time render is called update is also called
``` rust
renderer.render(&mut main_scene);
```


