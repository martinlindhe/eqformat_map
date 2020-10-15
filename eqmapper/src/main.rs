use drag_controller::{Drag, DragController};
use piston_window::Button::Keyboard;
use piston_window::Key;
use piston_window::*;

use clap::{App, Arg};

use eqformat_map::map::Map;

struct VisibleLayers {
    layers: [bool; 4],
}

fn main() {
    let matches = App::new("eqmapper")
        .version("0.1")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();

    let map = Map::from_file(filename);
    //let map = Map::from_file("C:/P2002_Titan/maps/poknowledge.txt");

    let mut visible_layers = VisibleLayers {
        layers: [true, true, true, true],
    };

    let (screen_width, screen_height) = (1100., 700.);

    let mut window: PistonWindow = WindowSettings::new("eqmapper", [screen_width, screen_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut drag = DragController::new();

    let mut offset_start_x = 0.;
    let mut offset_start_y = 0.;

    let mut offset_x = 0.;
    let mut offset_y = 0.;

    let mut zoom = 0.3;

    window.set_title(format!("eqmapper - {}", filename));

    window.set_lazy(true);
    while let Some(e) = window.next() {
        e.mouse_scroll(|pos| {
            // dy has huge values (+35 to -27) on macos, making scroll very jerky
            let mut step = pos[1];
            if step < -1. {
                step = -1.;
            } else if step > 1. {
                step = 1.;
            }
            zoom += step / 20.;
            if zoom < 0.1 {
                zoom = 0.1;
            } else if zoom > 2.0 {
                zoom = 2.0;
            }
            // println!("mouse_scroll {} (normalized to {}). zoom now = {}", pos[1], step, zoom);
        });
        drag.event(&e, |action| {
            match action {
                Drag::Start(x, y) => {
                    offset_start_x += x;
                    offset_start_y += y;
                    // println!("drag start at {}, {}", x, y);
                    true
                }
                Drag::Move(x, y) => {
                    // adjust transform based on drag
                    offset_x = x - offset_start_x;
                    offset_y = y - offset_start_y;
                    // println!("drag move at {}, {}. offset {}, {}", x, y, offset_x, offset_y);
                    true
                }
                Drag::End(x, y) => {
                    offset_start_x -= x;
                    offset_start_y -= y;
                    // println!("drag end at {}, {}", x, y);
                    false
                }
                // Continue dragging when receiving focus.
                Drag::Interrupt => true,
            }
        });
        if let Some(button) = e.press_args() {
            match button {
                Keyboard(Key::D0) => visible_layers.layers[0] = !visible_layers.layers[0],
                Keyboard(Key::D1) => visible_layers.layers[1] = !visible_layers.layers[1],
                Keyboard(Key::D2) => visible_layers.layers[2] = !visible_layers.layers[2],
                Keyboard(Key::D3) => visible_layers.layers[3] = !visible_layers.layers[3],
                Keyboard(Key::R) => {
                    // reset
                    for i in 0..=3 {
                        visible_layers.layers[i] = true;
                    }
                    zoom = 0.3;
                    offset_start_x = 0.;
                    offset_start_y = 0.;
                    offset_x = 0.;
                    offset_y = 0.;
                }
                _ => {}
            }
        }
        window.draw_2d(&e, |c, g, _device| {
            // make center of window the 0,0 coordinate
            let center = c.transform.trans(screen_width / 2., screen_height / 2.);

            clear([132. / 255., 106. / 255., 55. / 255., 1.0], g);

            // thicker lines for lower zoom levels
            let line_thickness = 1.0 / (zoom / 0.5);
            // println!("line thickness {} at zoom level {}", line_thickness, zoom);

            for layer in &map.layers {
                if visible_layers.layers[layer.id] {
                    // println!("Redrawing layer {}", layer.id);
                    for l in &layer.lines {
                        line(
                            [l.r, l.g, l.b, 1.0],
                            line_thickness,
                            [l.x1, l.y1, l.x2, l.y2],
                            center.trans(offset_x, offset_y).zoom(zoom),
                            g,
                        );
                    }
                }
            }
        });
    }
}
