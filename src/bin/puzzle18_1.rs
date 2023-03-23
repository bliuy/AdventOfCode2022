use std::{collections::HashSet, io::BufRead};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum ReferenceAxis {
    x(u8),
    y(u8),
    z(u8),
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: u8,
    y: u8,
    z: u8,
}

type SurfaceCoord = (u8, u8);
type CubeSurfaces = [Surface; 6];
struct Cube(Coord);

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Surface {
    surface_coord: SurfaceCoord,
    reference_axis: ReferenceAxis,
}

impl Cube {
    fn generate_surfaces(self) -> CubeSurfaces {
        let coord = self.0;
        let top = Surface {
            surface_coord: (coord.x, coord.y),
            reference_axis: ReferenceAxis::z(coord.z + 1),
        };
        let bottom = Surface {
            surface_coord: (coord.x, coord.y),
            reference_axis: ReferenceAxis::z(coord.z),
        };
        let front = Surface {
            surface_coord: (coord.x, coord.z),
            reference_axis: ReferenceAxis::y(coord.y),
        };
        let back = Surface {
            surface_coord: (coord.x, coord.z),
            reference_axis: ReferenceAxis::y(coord.y + 1),
        };
        let left = Surface {
            surface_coord: (coord.y, coord.z),
            reference_axis: ReferenceAxis::x(coord.x),
        };
        let right = Surface {
            surface_coord: (coord.y, coord.z),
            reference_axis: ReferenceAxis::x(coord.x + 1),
        };
        let result = [top, bottom, left, right, front, back];
        result
    }
}

const REGEX_PATTERN: &str = "([0-9]+),([0-9]+),([0-9]+)";

fn main() {
    // Reading inputs
    let file = std::fs::File::open("inputs/alt/input18.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    let mut existing_surfaces = HashSet::new();
    let mut joined_surfaces = HashSet::new();

    // Compiling the regex pattern
    let pattern = regex::Regex::new(REGEX_PATTERN).unwrap();

    for rline in reader.lines() {
        let line = rline.unwrap();
        let matched = pattern.captures(&line).unwrap();

        let x: u8 = matched.get(1).unwrap().as_str().parse().unwrap();
        let y: u8 = matched.get(2).unwrap().as_str().parse().unwrap();
        let z: u8 = matched.get(3).unwrap().as_str().parse().unwrap();

        let cube = Cube(Coord { x: x, y: y, z: z });
        let cube_surfaces = cube.generate_surfaces();

        for cs in cube_surfaces {
            match existing_surfaces.contains(&cs) {
                true => {
                    existing_surfaces.remove(&cs);
                    joined_surfaces.insert(cs);
                }
                false => {
                    existing_surfaces.insert(cs);
                }
            }
        }
    }
    println!("Result: {}", existing_surfaces.len());
}
