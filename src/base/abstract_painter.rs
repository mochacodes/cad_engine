pub trait AbstractPainter {
    fn set_width();
    fn set_color();
    fn draw_line(x: f64, y: f64, x2: f64, y2: f64);
    fn draw_circle(x: f64, y: f64, radius: f64);
}