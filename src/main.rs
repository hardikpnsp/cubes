mod cube_grid;

extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};
use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let size = 10;
    let rotation_angle_speed = 0.025;

    let mut cube_vec = cube_grid::add_cube_3d_grid(&mut window, size);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rotation_angle_speed);

    while window.render() {
        for cube in &mut *cube_vec {
            cube.prepend_to_local_rotation(&rot);
        }
    }
}
