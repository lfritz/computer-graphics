use math;

// An RGB color value.
export type color = struct {
	red: u8,
	green: u8,
	blue: u8,
};

export const BLACK = color{red=0, green=0, blue=0};
export const WHITE = color{red=0xff, green=0xff, blue=0xff};

// Returns true if a and b are equal.
export fn eq(a: color, b: color) bool = {
	return a.red == b.red &&
		a.green == b.green &&
		a.blue == b.blue;
};

@test fn eq() void = {
	const a = color{red=0x01, green=0x02, blue=0x03};
	let b = a;               assert(eq(a, b));
	let b = a; a.red   += 1; assert(!eq(a, b));
	let b = a; a.green += 1; assert(!eq(a, b));
	let b = a; a.blue  += 1; assert(!eq(a, b));
};

// Add the RGB values of two colors, clamping values to the range [0, 255].
export fn add(a: color, b: color) color = {
	return color{
		red=add_clamp(a.red, b.red),
		green=add_clamp(a.green, b.green),
		blue=add_clamp(a.blue, b.blue),
	};
};

@test fn add() void = {
	const a = color{red=0x01, green=0x02, blue=0x03};
	const b = color{red=0x10, green=0x20, blue=0x30};
	const c = color{red=0x11, green=0x22, blue=0x33};
	assert(eq(add(a, b), c));
	assert(eq(add(b, a), c));
	assert(eq(add(a, WHITE), WHITE));
	assert(eq(add(WHITE, a), WHITE));
};

fn add_clamp(a: u8, b: u8) u8 = {
	let sum = a:int + b: int;
	if (sum > 0xff) {
		return 0xff;
	};
	return sum: u8;
};

@test fn add_clamp() void = {
	assert(add_clamp(0, 0) == 0);
	assert(add_clamp(2, 3) == 5);
	assert(add_clamp(0, 0xff) == 0xff);
	assert(add_clamp(0xff, 0) == 0xff);
	assert(add_clamp(0xff, 1) == 0xff);
	assert(add_clamp(1, 0xff) == 0xff);
	assert(add_clamp(0xff, 0xff) == 0xff);
};

// Multiply a color's component values by a constant, rounding the result to integer values and
// clamping values to the range [0, 255].
export fn mul(k: f64, c: color) color = {
	return color{
		red=mul_clamp(c.red, k),
		green=mul_clamp(c.green, k),
		blue=mul_clamp(c.blue, k),
	};
};

@test fn mul() void = {
	const a = color{red=0x01, green=0x02, blue=0x03};
	assert(eq(mul(1.0, a), a));
	assert(eq(mul(2.0, a), color{red=0x02, green=0x04, blue=0x06}));
	assert(eq(mul(300.0, a), WHITE));
};

fn mul_clamp(a: u8, k: f64) u8 = {
	let product = math::roundf64(a: f64 * k) : int;
	if (product < 0) { return 0; };
	if (product > 0xff) { return 0xff; };
	return product: u8;
};

@test fn mul_clamp() void = {
	assert(mul_clamp(1, -1.0)  == 0);
	assert(mul_clamp(1, 0.25)  == 0);
	assert(mul_clamp(1, 0.5)   == 1);
	assert(mul_clamp(1, 1.0)   == 1);
	assert(mul_clamp(1, 1.25)  == 1);
	assert(mul_clamp(1, 1.5)   == 2);
	assert(mul_clamp(1, 2.0)   == 2);
	assert(mul_clamp(1, 300.0) == 0xff);
};
