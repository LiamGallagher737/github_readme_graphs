use tera::{Context, Tera};

#[derive(Clone, Debug)]
pub struct Graph {
    pub title: String,
    pub points: Vec<Vec2>,
    pub color: String,
}

#[derive(Clone, Debug, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn to_svg(self) -> String {
        format!("L {} {}", self.x, self.y)
    }
}

impl Graph {
    pub fn svg(&self, tera: &Tera, width: usize, height: usize) -> Result<String, tera::Error> {
        let mut context = Context::new();

        let min_x = self.points.get(0).map(|val| val.x).unwrap_or(0.0);
        let max_x = self
            .points
            .iter()
            .map(|val| (val.x - min_x))
            .fold(f64::NAN, f64::max);
        let min_y = self.points.iter().map(|val| val.y).fold(f64::NAN, f64::min);
        let max_y = self
            .points
            .iter()
            .map(|val| (val.y - min_y))
            .fold(f64::NAN, f64::max);

        let max_label = (max_y + min_y).to_string();

        //sometimes the min_y label is larger
        let min_label = min_y.to_string();

        //left padding changes based upon how many chars are, using a *really* rough 1 char * 9px;
        let p_left = (max_label.len() as f64 * 9.0 + 5.0)
            .max(min_label.len() as f64 * 9.0 + 5.0)
            .max(50.0);

        let p_other = 50.0;

        let width = width as f64 - p_left - p_other;
        let height = height as f64 - p_other * 2.0;

        let points: Vec<Vec2> = self
            .points
            .iter()
            .map(|val| Vec2 {
                x: ((val.x - min_x) / max_x * width) + p_left,
                y: ((val.y - min_y) / max_y * (height * -1.0)) + p_other + height,
            })
            .collect();

        let path = points
            .iter()
            .map(|val| val.to_svg())
            .collect::<Vec<String>>()
            .join("");

        context.insert("path", &path);
        context.insert("name", &self.title);
        context.insert("width", &(width));
        context.insert("height", &(height));
        context.insert("p_left", &p_left);
        context.insert("p_other", &p_other);
        context.insert("max_y", &max_y);
        context.insert("min_y", &min_y);
        context.insert("max_x", &max_x);
        context.insert("min_x", &min_x);
        context.insert("lines", &5);
        context.insert("color", &self.color);

        tera.render("graph", &context)
    }
}
