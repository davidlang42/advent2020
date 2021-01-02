use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;

const NEW_LINE: &str = "\r\n";

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Copy, Clone)]
enum Cube {
    Active,
    Inactive
}

impl Cube {
    fn from_char(c: &char) -> Self {
        match *c {
            '#' => Cube::Active,
            _ => Cube::Inactive
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cube::Active => '#',
            Cube::Inactive => '.',
        }
    }

    fn is_active(&self) -> bool {
        match self {
            Cube::Active => true,
            _ => false
        }
    }
}

struct PocketDimension {
    cubes: HashMap<Point,Cube>
}

impl PocketDimension {
    fn cycle(&mut self) {
        let mut new_cubes: HashMap<Point,Cube> = HashMap::new();
        let (min,max) = self.get_bounds();
        for x in (min.x-1)..(max.x+2) {
            for y in (min.y-1)..(max.y+2) {
                for z in (min.z-1)..(max.z+2) {
                    let point = Point { x,y,z };
                    let adjacent_active_count = self.get_adjacent_cubes(&point).iter().filter(|c| c.is_active()).count();
                    match self.get_cube(&point) {
                        Cube::Active => {
                            if adjacent_active_count == 2 || adjacent_active_count == 3 {
                                new_cubes.insert(point, Cube::Active);
                            }
                        },
                        Cube::Inactive => {
                            if adjacent_active_count == 3 {
                                new_cubes.insert(point, Cube::Active);
                            }
                        }
                    }
                }
            }
        }
        self.cubes = new_cubes;
    }

    fn get_adjacent_cubes(&self, point: &Point) -> Vec<&Cube> {
        let mut adjacent_cubes = Vec::new();
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                for z_offset in -1..2 {
                    if x_offset != 0 || y_offset != 0 || z_offset != 0 {
                        adjacent_cubes.push(self.get_cube(&Point {
                            x: point.x + x_offset,
                            y: point.y + y_offset,
                            z: point.z + z_offset
                        }));
                    }
                }
            }
        }
        adjacent_cubes
    }

    fn get_cube(&self, point: &Point) -> &Cube {
        match self.cubes.get(point) {
            Some(cube) => cube,
            None => &Cube::Inactive
        }
    }

    fn get_active_points(&self) -> HashSet<Point> {
        self.cubes.iter().filter(|(_p,c)| c.is_active()).map(|(p,_c)| *p).collect()
    }

    fn to_plane(&self, z_plane: isize) -> String {
        let (min,max) = self.get_bounds();
        let active_points = self.get_active_points();
        let mut s = String::new();
        for y in min.y..(max.y+1) {
            for x in min.x..(max.x+1) {
                if active_points.contains(&Point {x,y,z:z_plane}) {
                    s.push(Cube::Active.to_char());
                } else {
                    s.push(Cube::Inactive.to_char());
                }
            }
            s.push_str(NEW_LINE);
        }
        s
    }

    fn to_string(&self) -> String {
        let (min,max) = self.get_bounds();
        let mut s = String::new();
        for z in min.z..(max.z+1) {
            s.push_str(&format!("z={}",z));
            s.push_str(NEW_LINE);
            s.push_str(&self.to_plane(z));
            s.push_str(NEW_LINE);
        }
        s
    }

    fn get_bounds(&self) -> (Point, Point) {
        let active_points = self.get_active_points();
        let min = Point {
            x: active_points.iter().map(|p| p.x).min().unwrap(),
            y: active_points.iter().map(|p| p.y).min().unwrap(),
            z: active_points.iter().map(|p| p.z).min().unwrap()
        };
        let max = Point {
            x: active_points.iter().map(|p| p.x).max().unwrap(),
            y: active_points.iter().map(|p| p.y).max().unwrap(),
            z: active_points.iter().map(|p| p.z).max().unwrap()
        };
        (min,max)
    }

    fn new(plane: &Vec<Vec<Cube>>) -> Self {
        let mut pocket_dimension = PocketDimension {
            cubes: HashMap::new()
        };
        for (y,row) in plane.iter().enumerate() {
            for (x,cube) in row.iter().enumerate() {
                pocket_dimension.cubes.insert(Point {
                    x: x as isize,
                    y: y as isize,
                    z: 0
                }, *cube);
            }
        }
        pocket_dimension
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let cycles: usize = args[2].parse().unwrap();
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let initial_plane: Vec<Vec<Cube>> = text.split(NEW_LINE).map(|s| s.chars().map(|c| Cube::from_char(&c)).collect()).collect();
        let mut space = PocketDimension::new(&initial_plane);
        println!("Before any cycles:\r\n\r\n{}", space.to_string());
        for i in 0..cycles {
            space.cycle();
            println!("After {} cycles:\r\n\r\n{}", i+1, space.to_string());
        }
        println!("Result: {} active after {} cycles", space.get_active_points().iter().count(), cycles);
    } else {
        println!("Please provide 2 arguments: Filename, Cycles");
    }
}