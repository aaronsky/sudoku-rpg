use ggez::graphics::{Point2, Rect};

pub fn center_rect_in_rect(rect1: Rect, rect2: Rect) -> Point2 {
    let hpadding = (rect2.w - rect1.w) / 2.0;
    let vpadding = (rect2.h - rect1.h) / 2.0;
    Point2::new(rect2.x + hpadding, rect2.y + vpadding)
}

// pub fn center_rect_horizontally(rect: Rect, width: f32) -> Point2 {
//     let hpadding = (width - rect.w) / 2.0;
//     Point2::new(rect.x + hpadding, rect.y)
// }

pub fn center_rect_vertically(rect: Rect, height: f32) -> Point2 {
    let vpadding = (height - rect.h) / 2.0;
    Point2::new(rect.x, rect.y + vpadding)
}
