/*
 *  This file is part of Mendel Vectorizer.
 *
 *  Mendel Vectorizer is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mendel Vectorizer is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mendel Vectorizer.  If not, see <https://www.gnu.org/licenses/>.
*/


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
