extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::conrod::{self, widget, UiCell};
use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView, Camera};
use na::{Translation3, UnitQuaternion, Vector3, Point3};

use std::f32::consts::PI;
use std::path::Path;

use conrod::{Sizeable, Positionable, Labelable, Widget, widget_ids};

const UI_W: f64 = 300.;

struct Rotation {
    x_angle: f32,
    y_angle: f32,
    z_angle: f32,
}

impl Rotation {
    fn x(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.x_angle / 180.0 * PI)
    }

    fn y(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.y_angle / 180.0 * PI)
    }

    fn z(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.z_angle / 180.0 * PI)
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        slider_x,
        slider_y,
        slider_z,
        angle_x,
        angle_y,
        angle_z,
    }
}

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

// nomalized witdh
fn w_n(target_w: f32, window_h: f32) -> f32 {
    target_w / window_h
}

fn draw_ui(ui_cell: &mut UiCell, ids: &Ids, rot: &mut Rotation) {
    widget::Canvas::new()
        .align_left()
        .w(UI_W)
        .scroll_kids_vertically()
        .set(ids.canvas, ui_cell);

    for v in widget::Slider::new(rot.x_angle, -180.0, 180.0)
        .label("X")
        .w(200.)
        .h(25.)
        .mid_top_of(ids.canvas)
        .set(ids.slider_x, ui_cell) 
    {
        rot.x_angle = v;
    }

    widget::Text::new(&rot.x_angle.to_string())
        .right_from(ids.slider_x, 10.)
        .set(ids.angle_x, ui_cell);

    for v in widget::Slider::new(rot.y_angle, -180.0, 180.0)
        .label("Y")
        .w(200.)
        .h(25.)
        .down_from(ids.slider_x, 20.)
        .set(ids.slider_y, ui_cell) 
    {
        rot.y_angle = v;
    }

    widget::Text::new(&rot.y_angle.to_string())
        .right_from(ids.slider_y, 10.)
        .set(ids.angle_y, ui_cell);

    for v in widget::Slider::new(rot.z_angle, -180.0, 180.0)
        .label("Z")
        .w(200.)
        .h(25.)
        .down_from(ids.slider_y, 20.)
        .set(ids.slider_z, ui_cell) 
    {
        rot.z_angle = v;
    }

    widget::Text::new(&rot.z_angle.to_string())
        .right_from(ids.slider_z, 10.)
        .set(ids.angle_z, ui_cell);
}

fn main() {
    // Window
    let mut window = Window::new("Kiss3d: obj");
    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    // State
    let mut rotation = Rotation {x_angle: 0.0, y_angle: 0.0, z_angle: 0.0};

    // Teapot
    let obj_path = Path::new("./src/media/teapot.obj");
    let mtl_path = Path::new("./src/media");
    println!("{:?}", obj_path);
    println!("{:?}", mtl_path);
    let mut teapot = window.add_obj(obj_path, mtl_path, Vector3::new(0.001, 0.001, 0.001));
    
    // UI
    window.conrod_ui_mut().theme = theme();
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    while window.render() {
        let window_w = window.width() as f32;
        let window_h = window.height() as f32;

        let teapot_x_translation = -w_n(UI_W as f32, window_h) / 2.;
        
        teapot.set_local_translation(Translation3::new(teapot_x_translation, 0.0, 0.0));
        teapot.set_local_rotation(rotation.x() * rotation.y() * rotation.z());

        let mut ui_cell = window.conrod_ui_mut().set_widgets();
        draw_ui(&mut ui_cell, &ids, &mut rotation);
    }
}