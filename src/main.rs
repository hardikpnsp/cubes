extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut c = add_red_cube(
        &mut window,
        Translation3::new(0.0, 0.0, 0.0)
    );

    let rot = UnitQuaternion::from_axis_angle(
        &Vector3::y_axis(),
        0.014
    );
    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}

fn add_red_cube(window: &mut Window, cube_translation: Translation3<f32>) -> SceneNode {
    let mut c = window.add_cube(1.0, 1.0, 1.0);
    c.set_color(1.0, 0.0, 0.0);
    c.set_local_transformation(Isometry3 {
        rotation: (UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0f32)),
        translation: cube_translation,
    });
    c
}
