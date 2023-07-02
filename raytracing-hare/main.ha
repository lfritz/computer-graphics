use fs;
use io;
use os;

use canvas;

export fn main() void = {
	let c = canvas::new(640, 480);
	defer canvas::finish(&c);

	const mode = fs::mode::USER_RW | fs::mode::GROUP_R | fs::mode::OTHER_R | fs::mode::REG;
	const file = os::create("image.ppm", mode, fs::flag::WRONLY)!;
	defer io::close(file)!;
	canvas::write(file, &c)!;
};
