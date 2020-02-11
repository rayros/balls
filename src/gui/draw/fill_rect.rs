use stdweb::web::CanvasRenderingContext2d;

pub fn fill_rect(ctx: &CanvasRenderingContext2d, x: i32, y: i32, width: i32, height: i32) {
  ctx.fill_rect(
    f64::from(x),
    f64::from(y),
    f64::from(width),
    f64::from(height),
  );
}