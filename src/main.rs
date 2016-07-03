extern crate image;

use image::GenericImage;
use image::Rgb;
use std::env;
use std::path::Path;

// ocs: other components strength
// scr: scanline reducing factor
fn make_rgb_grid(ocs: u8, scrf: u8) -> [[[u8; 3]; 6]; 8] {
	let sc2 = 255u8.saturating_sub(scrf);
	let sc1 = ocs.saturating_sub(scrf);

	let ret: [[[u8; 3]; 6]; 8] = [
		[[255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255], [255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255]],
		[[255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255], [sc2,sc1,sc1], [sc1,sc2,sc1], [sc1,sc1,sc2]],
		[[255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255], [255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255]],
		[[sc2,sc1,sc1], [sc1,sc2,sc1], [sc1,sc1,sc2], [255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255]],
		[[255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255], [255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255]],
		[[255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255], [sc2,sc1,sc1], [sc1,sc2,sc1], [sc1,sc1,sc2]],
		[[255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255], [255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255]],
		[[sc2,sc1,sc1], [sc1,sc2,sc1], [sc1,sc1,sc2], [255,ocs,ocs], [ocs,255,ocs], [ocs,ocs,255]],
	];
	ret
}

fn main() {
	let mut args = env::args();
	if args.len() == 1 {
		println!("scanlines v1 - Hadrien Nilsson - 2016");
		println!("Convert an image as if it was displayed on a CRT display.");
		println!("usage: scanlines INFILE [OUTFILE]");
		println!("If OUTFILE is missing, a suffix is appended to INFILE for the new file name.");
		println!("The result is saved as a PNG file.");
		std::process::exit(0);
	}
	args.next().unwrap(); // Skip programe name

	let in_file_path = match args.next() {
		Some(arg) => arg,
		None => {
			println!("Missing input file");
			std::process::exit(1);
		}
	};

	let out_file_path = match args.next() {
		Some(arg) => arg,
		None => {
			let mut s = String::from(Path::new(&in_file_path).file_stem().unwrap().to_str().unwrap());
			s.push_str("-scanlines.png");
			s
		}
	};

    let src_img = match image::open(in_file_path) {
    	Ok(img) => img,
    	Err(err) => {
    		println!("image open failed: {}", err);
    		std::process::exit(1);
    	}
    };

    let rgb_grid = make_rgb_grid(180, 50);

	let (src_width, src_height) = src_img.dimensions();
	let dst_width = src_width * 4;
	let dst_height = src_height * 4;
	let mut dst_img = image::RgbImage::new(dst_width, dst_height);

	let mut grid_x;
	let mut grid_y = 0usize;
	
	let mut prev_src_rgb: [i32; 3];

	for y in 0..src_height {
		grid_x = 0;
		let grid_y_bak = grid_y;
		prev_src_rgb = [0; 3];

		for x in 0..src_width {
			let src_px = src_img.get_pixel(x, y);
			let src_rgb = [
				src_px.data[0] as i32,
				src_px.data[1] as i32, 
				src_px.data[2] as i32
			];

			let grid_x_bak = grid_x;
			grid_y = grid_y_bak;

			for j in 0..4 {
				grid_x = grid_x_bak;
				for i in 0..4 {
					
					let mut new_rgb: [u8; 3] = [0; 3];
					// For each RGB component
					for k in 0..3 {
						// The CRT beam intensity changes progressively
						// Blur with previous horizontal pixels
						let mut comp = if i == 0 {
							(src_rgb[k] + prev_src_rgb[k]) / 2
						}
						else if i == 1 {
							(src_rgb[k]*3 + prev_src_rgb[k]) / 4
						}
						else { src_rgb[k] };

						// There is a gap between each scanline
						if j == 0 {
							comp -= 60;
							if comp < 0 { comp = 0; }
						}

						if j == 3 {
							comp -= 120;
							if comp < 0 { comp = 0; }
						}

						// Apply the RGB grid
						let grid_comp = rgb_grid[grid_y][grid_x][k] as i32;
						
						// Increase power for high brightness pixels
						// Divide by less than 255 to increase general brightness
						comp = ((comp + comp/8) * grid_comp) / 200;
						if comp < 0 { comp = 0; }
						if comp > 255 { comp = 255; }

						new_rgb[k] = comp as u8;
					}

					//let px = Rgb::from_slice(&new_rgb);
					let px = Rgb { data: new_rgb };
					dst_img.put_pixel(4*x+i, 4*y+j, px);

					grid_x = if (grid_x + 1) == 6 { 0 } else { grid_x + 1};
				}
				grid_y = if (grid_y + 1) == 8 { 0 } else { grid_y + 1};
			}
			prev_src_rgb = src_rgb;
		}
	}

    if let Err(err) = dst_img.save(out_file_path) {
		println!("image save failed: {}", err);
		std::process::exit(1);
    }
}
