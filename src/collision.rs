use cgmath::Zero;

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

pub fn circle_intersects_rectangle(circle: &Circle, rectangle: &Rectangle) -> Option<Vector2<f32>>
{
    let mut vertices: Vec<Vector2<f32>> = Vec::new();

    vertices.push(Vector2::new(rectangle.left, rectangle.top));
    vertices.push(Vector2::new(rectangle.right, rectangle.top));
    vertices.push(Vector2::new(rectangle.right, rectangle.bottom));
    vertices.push(Vector2::new(rectangle.left, rectangle.bottom));

    let mut is_outside = false;
    let mut min_current_vertex: Vector2<f32> = Vector2::zero();
    let mut min_next_vertex: Vector2<f32> = Vector2::zero();
    let mut distance_circle_edge = f32::MIN;
    let mut max_projection = f32::MIN;

    for i in 0.. vertices.len()
    {
        let current_vertex = i;
        let next_vertex = (i + 1) % vertices.len();

        let edge = vertices[next_vertex] - vertices[current_vertex];
        let normal = normal(&edge);

        let vertex_to_circle_centre = circle.centre - vertices[current_vertex];
        let projection = vertex_to_circle_centre.dot(normal);

        if projection > 0.0 && projection > max_projection
        {
            max_projection = projection;
            distance_circle_edge = projection;
            min_current_vertex = vertices[current_vertex];
            min_next_vertex = vertices[next_vertex];
            is_outside = true;
        }
        else if projection > distance_circle_edge
        {
            distance_circle_edge = projection;
            min_current_vertex = vertices[current_vertex];
            min_next_vertex = vertices[next_vertex];
        }
    }

    if is_outside
    {
        let mut v1 = circle.centre - min_current_vertex;
        let mut v2  = min_next_vertex - min_next_vertex;

        if v1.dot(v2) < 0.0
        {
            if v1.magnitude() > circle.radius {
                return None;
            }
            else {
                return Some(v1.normalize() * (circle.radius - v1.magnitude()));
            }
        }
        else
        {
            v1 = circle.centre - min_next_vertex;
            v2 = min_current_vertex - min_next_vertex;

            if v1.dot(v2) < 0.0
            {
                if v1.magnitude() > circle.radius {
                    return None;
                }
                else {
                    return Some(v1.normalize() * (circle.radius - v1.magnitude()));
                }
            }
            else 
            {
                if distance_circle_edge > circle.radius {
                    return None;
                }
                else 
                {
                    let normal = normal(&(min_next_vertex - min_current_vertex));
                    return Some(normal * (circle.radius - distance_circle_edge));
                }
            }
        }
    }

    let normal = normal(&(min_next_vertex - min_current_vertex));
    return Some(normal * (circle.radius - distance_circle_edge));
}

fn normal(vector: &Vector2<f32>) -> Vector2<f32> {
    Vector2::new(vector.y, -vector.x).normalize()
}