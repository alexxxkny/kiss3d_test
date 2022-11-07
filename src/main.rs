extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::conrod::{self, widget};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Translation3, UnitQuaternion, Vector3, Point3};
use std::f32;
use std::path::Path;

use conrod::{Sizeable, Positionable, Labelable, Widget, widget_ids};

widget_ids! {
    pub struct Ids {
        canvas,
        range,
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

fn main() {
    let mut window = Window::new("Kiss3d: obj");

    // Teapot
    let obj_path = Path::new("./src/media/teapot.obj");
    let mtl_path = Path::new("./src/media");
    println!("{:?}", obj_path);
    println!("{:?}", mtl_path);
    let mut teapot = window.add_obj(obj_path, mtl_path, Vector3::new(0.001, 0.001, 0.001));
    
    let rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 2.);
    
    //teapot.set_local_rotation(rot);
    //teapot.set_local_translation(Translation3::new(-0.4, 0.0, 0.0));

    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    let rot_teapot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    //window.conrod_ui_mut().theme = theme();
    let ui_canvas_width = 300.0;

    while window.render() {
        let window_width = window.width() as f64;
        let mut ui = window.conrod_ui_mut().set_widgets();

        let teapot_x_translation = (-1.0 + (1.0 * (ui_canvas_width / window_width) / 2.0)) as f32;
        println!("{}", teapot_x_translation);
        //teapot.set_local_translation(Translation3::new(teapot_x_translation, 0.0, 0.0));
        teapot.set_local_translation(Translation3::new(2.0, 0.0, 0.0));

        widget::Canvas::new()
            .align_left()
            .w(300.)
            .scroll_kids_vertically()
            .set(ids.canvas, &mut ui);
        
        widget::RangeSlider::new(1., 10., 1., 10.)
            .label("X")
            .w(200.)
            .h(25.)
            .mid_top_of(ids.canvas)
            .set(ids.range, &mut ui);

        drop(ui);

        window.draw_line(&Point3::new(0.0, 0.5, 0.0), &Point3::new(0.0, -0.5, 0.0), &Point3::new(0.0, 0.0, 0.0));
        window.draw_line(&Point3::new(0.5, 0.0, 0.0), &Point3::new(-0.5, 0.0, 0.0), &Point3::new(0.0, 0.0, 0.0));
    }
}