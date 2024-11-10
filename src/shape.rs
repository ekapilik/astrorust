use macroquad::prelude::*;

pub struct Shape {
    pub points: Vec<Vec2>,
    pub color: Color,
    pub thickness: f32,
}

fn rotate_point(base: Vec2, end: Vec2, rotation: f32) -> Vec2 {
    // https://math.stackexchange.com/questions/270194/how-to-find-the-vertices-angle-after-rotation
    let dx = end.x - base.x;
    let dy = end.y - base.y;
    let x = base.x + dx * rotation.cos() - dy * rotation.sin();
    let y = base.y + dx * rotation.sin() + dy * rotation.cos();
    return vec2(x, y);
}

impl Shape {
    fn transform(&self, center: Vec2, rotation: f32) -> Vec<Vec2> {
        let translated_points: Vec<Vec2> =
            self.points.iter().map(|point| *point + center).collect();
        let rotated_points: Vec<Vec2> = translated_points
            .iter()
            .map(|point| rotate_point(center, *point, rotation))
            .collect();
        return rotated_points;
    }

    pub fn draw(&self, center: Vec2, rotation: f32) {
        let transformed_points = self.transform(center, rotation);
        let mut prev_point = transformed_points.first().unwrap();
        let remaining_points = &transformed_points[1..];
        for point in remaining_points {
            draw_line(
                prev_point.x,
                prev_point.y,
                point.x,
                point.y,
                self.thickness,
                self.color,
            );
            prev_point = point;
        }
    }
}
