use macroquad::prelude::*;

pub fn is_colliding(polygon_a: &Vec<Vec2>, polygon_b: &Vec<Vec2>) -> bool {
    // 2D physics collision detection
    // assume points create a concave polygon
    for point_a in polygon_a.iter() {
        if point_in_polygon(point_a, polygon_b) {
            return true;
        }
    }
    return false;
}

pub fn point_in_polygon(point: &Vec2, polygon: &Vec<Vec2>) -> bool {
    // https://en.wikipedia.org/wiki/Point_in_polygon
    let x = point.x;
    let y = point.y;
    let mut inside = false;

    let mut point_1 = polygon.first().unwrap();
    for point_2 in polygon.iter().skip(1) {
        if y > point_1.y.min(point_2.y) {
            if y <= point_1.y.max(point_2.y) {
                if x <= point_1.x.max(point_2.x) {
                    let x_intercept = if point_1.y != point_2.y {
                        (y - point_1.y) * (point_2.x - point_1.x) / (point_2.y - point_1.y)
                            + point_1.x
                    } else {
                        point_1.x
                    };
                    if point_1.x == point_2.x || x <= x_intercept {
                        inside = !inside;
                    }
                }
            }
        }
        point_1 = point_2;
    }
    inside
}
