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


use image::imageops::colorops::grayscale;
use imageproc::corners::{corners_fast9, Corner};

pub fn fast9(img: String) -> Vec<Corner> {
    let img = image::open(img).unwrap();
    let img = grayscale(&img);
    corners_fast9(&img,50)
}
