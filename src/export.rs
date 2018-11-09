use crate::bezier::Bezier;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub fn export(lines: &[Bezier], filename: PathBuf) {
    let mut svg = String::from("<svg width=\"\" height=\"\" xmlns=\"http://www.w3.org/2000/svg\">");
    for line in lines {
        svg += &format!("<path d=\"M{} {} C {} {}, {} {}, {} {}\" style=\"stroke: black;fill:none\"/>",
            line.start.x,line.start.y,
            line.control1.x,line.control1.y,
            line.control2.x,line.control2.y,
            line.end.x,line.end.y
        );
    }
    svg += "</svg>";

    let mut file = File::create(filename).unwrap();
    file.write_all(svg.as_bytes()).unwrap();
}