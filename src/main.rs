use kiss3d::nalgebra::{Vector3, UnitQuaternion, Translation};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use std::io::BufRead;
use std::vec::Vec;

type Arr3 = [f32; 3];

const color_array : [Arr3; 3] = [
    [0.9, 0.1, 0.1],
    [0.1, 0.1, 0.9],
    [0.1, 0.9, 0.1]
];

fn add_array_3(this: &Arr3, other: &Arr3) -> Arr3 {
    [this[0] + other[0], this[1] + other[1], this[2] + other[2]]
}

#[derive(Debug, Default)]
struct State {
    box_list: Vec<(Arr3, Arr3)> // (Position, Color)
}

impl State {
    fn get_next_color(&self) -> &Arr3 {
        let index = (self.box_list.len() + 1) % color_array.len();
        &color_array[index]
    }

    fn push_box(&mut self, dir: &[f32; 3], node: &mut SceneNode, win: &mut Window) {
        static mut color : i32 = 0;
        type Trans = Translation::<f32, 3>;

        let accum_tr = if let Some(val) = self.box_list.last() {
            add_array_3(dir, &val.0)
        } else {
            [0.0, 0.0, 0.0]
        };

        let next_color = self.get_next_color();
        let next_color_clone : &Arr3 = &next_color.clone();
        self.box_list.push((accum_tr, *next_color));
        node.set_color(next_color_clone[0], next_color_clone[1], next_color_clone[2]);
        node.append_translation(&Trans::new(accum_tr[0], accum_tr[1], accum_tr[2]));
        *node = win.add_cube(0.2, 0.2, 0.2);
    }

}

fn init_layout() -> Window {
    let mut buffer = String::with_capacity(15);
    let mut stdin = std::io::stdin().lock();
    stdin.read_line(&mut buffer);


    let mut window = Window::new("Spiderbox");
    let mut state = State::default();
    let mut node = SceneNode::new_empty();

    for dir in buffer.chars() {
        match dir {
            'x' => state.push_box(&[0.2, 0.0, 0.0], &mut node, &mut window),
            'y' => state.push_box(&[0.0, 0.2, 0.0], &mut node, &mut window),
            'z' => state.push_box(&[0.0, 0.0, 0.2], &mut node, &mut window),
            _ => println!("Only x, y and z are allowed. Ignoring illegal char {}", dir),
        }
    }
    println!("{:?}", state);
    window.set_light(Light::StickToCamera);
    window
}

fn main() {
    let mut window = init_layout();


    // let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        // c.prepend_to_local_rotation(&rot);
    }
}