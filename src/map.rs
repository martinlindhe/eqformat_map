use std::path::Path;

use crate::string::{file_as_string, str_as_f64, str_as_u8};

#[derive(Debug)]
pub struct Map {
    pub layers: Vec<MapLayer>,
}

#[derive(Debug)]
pub struct MapLayer {
    pub id: usize, // 0 = base
    pub labels: Vec<MapLabel>,
    pub lines: Vec<MapLine>,
}

#[derive(Debug, PartialEq)]
pub struct MapLabel {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    size: u8,
    text: String,
}

#[derive(Debug, PartialEq)]
pub struct MapLine {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Map {
    pub fn default() -> Self {
        Self { layers: Vec::new() }
    }

    /// filename should be the map base layer, extra layers will automatically be detected
    pub fn from_file(filename: &str) -> Self {
        let mut map = Self::default();
        map.read_layer(0, filename);

        for i in 1..=3 {
            let layerfile = Self::build_layer_filename(i, filename);
            map.read_layer(i as usize, &layerfile);
        }
        map
    }

    fn build_layer_filename(layer_id: u8, filename: &str) -> String {
        let path = Path::new(filename).canonicalize().unwrap();
        let parent = path.parent().unwrap();
        let basename = path.file_stem().unwrap();
        let tmp = parent.join(basename);
        let tmp = tmp.to_str().unwrap();
        format!("{}_{}.txt", tmp, layer_id)
    }

    fn read_layer(&mut self, layer_id: usize, filename: &str) {
        let mut layer = MapLayer::default();
        layer.id = layer_id;
        if let Some(s) = file_as_string(filename) {
            for line in s.lines() {
                layer.parse_row(line);
            }
            println!(
                "Read map layer {} from {} with {} bytes",
                layer_id,
                filename,
                s.len()
            );
            self.layers.push(layer);
        }
    }
}

impl MapLayer {
    pub fn default() -> Self {
        Self {
            id: 0,
            labels: Vec::new(),
            lines: Vec::new(),
        }
    }

    fn parse_row(&mut self, s: &str) {
        if let Some(c) = s.get(0..2) {
            match c {
                "P " => {
                    // Label
                    if let Some(label) = parse_eqmap_label(&s[2..]) {
                        self.labels.push(label);
                    } else {
                        println!("XXX invalid LABEL: {}", s);
                    }
                }
                "L " => {
                    // Line
                    if let Some(line) = parse_eqmap_line(&s[2..]) {
                        self.lines.push(line);
                    } else {
                        println!("XXX invalid LINE: {}", s);
                    }
                }
                _ => println!("XXX unhandled map format: {}    {}", c, s),
            }
        }
    }
}

fn parse_eqmap_label(s: &str) -> Option<MapLabel> {
    // format: X, Y, Z, R, G, B, size, text
    let parts: Vec<&str> = s.splitn(8, ',').collect();
    if parts.len() != 8 {
        return None;
    }
    Some(MapLabel {
        x: str_as_f64(parts[0]),
        y: str_as_f64(parts[1]),
        z: str_as_f64(parts[2]),
        r: f32::from(str_as_u8(parts[3])) / 255.,
        g: f32::from(str_as_u8(parts[4])) / 255.,
        b: f32::from(str_as_u8(parts[5])) / 255.,
        size: str_as_u8(parts[6]),
        text: parts[7].trim().to_owned(),
    })
}

fn parse_eqmap_line(s: &str) -> Option<MapLine> {
    // format: X1, Y1, Z1, X2, Y2, Z2, R, G, B
    let parts: Vec<&str> = s.splitn(9, ',').collect();
    if parts.len() != 9 {
        return None;
    }
    Some(MapLine {
        x1: str_as_f64(parts[0]),
        y1: str_as_f64(parts[1]),
        z1: str_as_f64(parts[2]),
        x2: str_as_f64(parts[3]),
        y2: str_as_f64(parts[4]),
        z2: str_as_f64(parts[5]),
        r: f32::from(str_as_u8(parts[6])) / 255.,
        g: f32::from(str_as_u8(parts[7])) / 255.,
        b: f32::from(str_as_u8(parts[8])) / 255.,
    })
}

#[test]
fn test_parse_eqmap_label() {
    assert_eq!(
        MapLabel {
            x: 5531.2642,
            y: -168.7061,
            z: -299.5485,
            r: 0.5019607843137255,
            g: 1.,
            b: 0.,
            size: 2,
            text: "Gargoyle_Island".to_owned()
        },
        parse_eqmap_label("5531.2642, -168.7061, -299.5485,  128, 255, 0,  2,  Gargoyle_Island").unwrap()
    );

    // label can contain commas
    assert_eq!(
        MapLabel {
            x: -3710.0198,
            y: -1594.5485,
            z: -192.5240,
            r: 0.5019607843137255,
            g: 1.,
            b: 0.,
            size: 2,
            text: "Gull_Skytalon_(Named,Roam)".to_owned()
        },
        parse_eqmap_label("-3710.0198, -1594.5485, -192.5240,  128, 255, 0,  2,  Gull_Skytalon_(Named,Roam)").unwrap()
    );
}

#[test]
fn test_parse_eqmap_line() {
    assert_eq!(
        MapLine {
            x1: 2881.0,
            y1: -2022.0,
            z1: -295.0,
            x2: 2885.0,
            y2: -2027.0,
            z2: -295.0,
            r: 0.5019607843137255,
            g: 1.0,
            b: 0.0
        },
        parse_eqmap_line("2881.0, -2022.0, -295.0, 2885.0, -2027.0, -295.0, 128, 255, 0").unwrap()
    );
}
