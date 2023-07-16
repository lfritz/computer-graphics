use bufio;
use fmt;
use io;
use strings;

use color;

// A 2D grid of pixels.
export type canvas = struct {
	width: size,
	height: size,
	data: []u8,
};

// Creates a new [[canvas]] with all pixels set to black. The user must free resources associated
// using [[finish]] after use.
export fn new(width: size, height: size) canvas = {
	const n: size = width * height * 3;
	const data = alloc([0u8...], n);
	return canvas{width=width, height=height, data=data};
};

// Frees resources associated with a [[canvas]].
export fn finish(c: *canvas) void = {
	free(c.data);
};

// Returns true if a and b are the same size and contain the same pixels.
export fn eq(a: *canvas, b: *canvas) bool = {
	if (!(a.width == b.width && a.height == b.height)) {
		return false;
	};
	const n = len(a.data);
	for (let i: size = 0; i < n; i += 1) {
		if (a.data[i] != b.data[i]) {
			return false;
		};
	};
	return true;
};

@test fn eq() void = {
	let   a = new(3, 2); defer finish(&a);
	const b = new(3, 2); defer finish(&b);
	const c = new(2, 3); defer finish(&c);
	assert(eq(&a, &b));
	assert(!eq(&a, &c));
	putpixel(&a, 0, 0, color::color{red=0x00, green=0xff, blue=0xff});
	assert(!eq(&a, &b));
};

// Sets the pixel at (x, y) in the canvas. This uses a coordinate system where (0, 0) is the
// bottom-left pixel and x goes from left to right, and y from bottom to top.
export fn putpixel(c: *canvas, x: int, y: int, col: color::color) void = {
	const y = c.height: int - 1 - y;
	const base = ((y * c.width: int) + x) * 3;
	c.data[base + 0] = col.red;
	c.data[base + 1] = col.green;
	c.data[base + 2] = col.blue;
};

// Returns a canvas of size 3x2 to be used for testing. The pixel values are set to match the ASCII
// lower-case letters, which makes some of the test code a lot simpler.
fn testcanvas() canvas = {
	let c = new(3, 2);
	putpixel(&c, 0, 1, color::color{red=0x61, green=0x62, blue=0x63});
	putpixel(&c, 1, 1, color::color{red=0x64, green=0x65, blue=0x66});
	putpixel(&c, 2, 1, color::color{red=0x67, green=0x68, blue=0x69});
	putpixel(&c, 0, 0, color::color{red=0x6a, green=0x6b, blue=0x6c});
	putpixel(&c, 1, 0, color::color{red=0x6d, green=0x6e, blue=0x6f});
	putpixel(&c, 2, 0, color::color{red=0x70, green=0x71, blue=0x72});
	return c;
};

@test fn putpixel() void = {
	let c = testcanvas();
	defer finish(&c);
	const want = canvas{
		width=3,
		height=2,
		data=strings::toutf8("abcdefghijklmnopqr"),
	};
	assert(eq(&c, &want));
};

// Store the canvas in PPM format using the given handle.
export fn write(h: io::handle, c: *canvas) (void | io::error) = {
	fmt::fprintln(h, "P6")?;
	fmt::fprintfln(h, "{} {}", c.width, c.height)?;
	fmt::fprintln(h, "255")?;
	for (let buf = c.data; len(buf) > 0) {
		const n = io::write(h, buf)?;
		buf = buf[n..];
	};
};

@test fn write() void = {
	let c = testcanvas();
	defer finish(&c);

	const buf = bufio::dynamic(io::mode::RDWR);
	defer io::close(&buf)!;

	write(&buf, &c)!;
	const got = strings::fromutf8(bufio::buffer(&buf))!;
	const want = "P6\n3 2\n255\nabcdefghijklmnopqr";
	assert(got == want);
};
