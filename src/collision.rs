
use crate::{math::Vector2, shapes::rectangle::Rectangle};


pub fn rectangle_rectangle_intersects(rectangle_a: &Rectangle, rectangle_b: &Rectangle) -> Option<Vector2<f32>>
{ 
    let half_width_a = rectangle_a.width / 2.0;
    let half_height_a = rectangle_a.height / 2.0;

    let half_width_b = rectangle_b.width / 2.0;
    let half_height_b = rectangle_b.height / 2.0;

    let centre_a = Vector2::new(rectangle_a.left + half_width_a, rectangle_a.top + half_height_a);
    let centre_b = Vector2::new(rectangle_b.left + half_width_b, rectangle_b.top + half_height_b);

    let distance_x = centre_a.x - centre_b.x;
    let distance_y = centre_a.y - centre_b.y;

    let min_distance_x = half_width_a + half_width_b;
    let min_distance_y = half_height_a + half_height_b;

    if distance_x.abs() >= min_distance_x || distance_y.abs() >= min_distance_y {
        return None;
    }

    let mut depth_x = 0.0;
    let mut depth_y = 0.0;

    if distance_x > 0.0 {
        depth_x = min_distance_x - distance_x;
    }
    else {
        depth_x = -min_distance_x - distance_x;
    }

    if distance_y > 0.0 {
        depth_y = min_distance_y - distance_y;
    }
    else {
        depth_y = -min_distance_y - distance_y;
    }

    Some(Vector2::new(depth_x, depth_y))
}
