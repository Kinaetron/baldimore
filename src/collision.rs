use crate::{math::Vector2, math::InnerSpace};
use crate::shapes::{rectangle::Rectangle, circle::Circle};


pub fn rectangle_intersects_rectangle(rectangle_a: &Rectangle, rectangle_b: &Rectangle) -> Option<Vector2<f32>>
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
    
    let depth_x = if distance_x > 0.0 { min_distance_x - distance_x } else { -min_distance_x - distance_x };
    let depth_y = if distance_y > 0.0 {  min_distance_y - distance_y } else { -min_distance_y - distance_y };

    Some(Vector2::new(depth_x, depth_y))
}

pub fn circle_intersects_circle(circle_a: &Circle, circle_b: &Circle) -> Option<Vector2<f32>>
{
    let radius_sum = circle_a.radius + circle_b.radius;
    let distance_vec = circle_a.centre - circle_b.centre;

    let distance = distance_vec.magnitude();

    if distance > radius_sum {
        return None;
    }

    let depth = radius_sum - distance;
    let direction = (circle_a.centre - circle_b.centre).normalize();

    let depth_vector = direction * depth;

    Some(depth_vector)
}