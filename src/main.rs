extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let size = 10;
    let rotation_angle_speed = 0.025;

    let mut cube_vec = add_cube_3d_grid(&mut window, size);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rotation_angle_speed);

    while window.render() {
        for cube in &mut *cube_vec {
            cube.prepend_to_local_rotation(&rot);
        }
    }
}

fn add_cube_3d_grid(mut window: &mut Window, size: i32) -> Vec<SceneNode> {
    let mut cube_vec: Vec<SceneNode> = Vec::new();

    for z in 0..size {
        cube_vec.append(&mut add_cube_2d_grid(
            &mut window,
            size,
            center_coordinate(size, z) as f32,
        ));
    }
    cube_vec
}

fn add_cube_2d_grid(mut window: &mut Window, size: i32, z: f32) -> Vec<SceneNode> {
    let mut cube_vec: Vec<SceneNode> = Vec::new();

    for i in 0..size {
        cube_vec.append(&mut add_cube_line(
            &mut window,
            size,
            center_coordinate(size, i) as f32,
            z,
        ));
    }
    cube_vec
}

fn add_cube_line(mut window: &mut Window, size: i32, y: f32, z: f32) -> Vec<SceneNode> {
    let mut cube_vec: Vec<SceneNode> = Vec::new();

    for i in 0..size {
        cube_vec.push(add_red_cube(
            &mut window,
            Translation3::new(center_coordinate(size, i) as f32, y, z),
        ));
    }
    cube_vec
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

fn center_coordinate(capacity: i32, coordinate: i32) -> i32 {
    2 * (coordinate - (capacity / 2))
}
