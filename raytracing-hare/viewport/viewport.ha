use canvas;
use fmt;
use vec3;

// The viewport is a rectangle in 3-D space through which we're looking at the scene. Pixels on
// the canvas can be mapped to points in the viewport.
export type viewport = struct {
	c: *canvas::canvas,
	f: f64, // ratio of viewport coordinates : canvas coordinates
	d: f64, // distance from origin
};

// Returns a new viewport for the given canvas, where w is the viewport's width and d is the
// distance from the origin. The viewport's height is calculated so the viewport's aspect ratio
// matches the canvas's.
export fn new(c: *canvas::canvas, w: f64, d: f64) viewport = {
	return viewport{
		c=c,
		f=w / c.width: f64,
		d=d,
	};
};

@test fn new() void = {
	const c = canvas::new(3, 2);
	defer canvas::finish(&c);
	const v = new(&c, 6.0, 2.5);
	assert(v.f == 2.0);
	assert(viewport_width(&v) == 6.0);
	assert(viewport_height(&v) == 4.0);
};

// Returns the width of the viewport.
fn viewport_width(v: *viewport) f64 = {
	return v.c.width: f64 * v.f;
};

// Returns the height of the viewport.
fn viewport_height(v: *viewport) f64 = {
	return v.c.height: f64 * v.f;
};

// Converts a pixel coordinate in the canvas (in the coordinate system used by [[canvas::putpixel]])
// to a point in the viewport.
fn canvas_to_viewport(v: *viewport, cx: int, cy: int) vec3::vec3 = {
	const x = (cx: f64 - (v.c.width  - 1): f64 / 2.0) * v.f;
	const y = (cy: f64 - (v.c.height - 1): f64 / 2.0) * v.f;
	return vec3::new(x, y, v.d);
};

@test fn canvas_to_viewport() void = {
	const c = canvas::new(3, 2);
	defer canvas::finish(&c);
	const v = new(&c, 6.0, 2.5);
	const cases: [](int, int, vec3::vec3) = [
		(0, 0, vec3::new(-2.0, -1.0, 2.5)),
		(1, 0, vec3::new( 0.0, -1.0, 2.5)),
		(2, 0, vec3::new( 2.0, -1.0, 2.5)),
		(0, 1, vec3::new(-2.0,  1.0, 2.5)),
		(1, 1, vec3::new( 0.0,  1.0, 2.5)),
		(2, 1, vec3::new( 2.0,  1.0, 2.5)),
	];
	for (let i: size = 0; i < len(cases); i += 1) {
		const (cx, cy, want) = cases[i];
		const got = canvas_to_viewport(&v, cx, cy);
		fmt::printfln("canvas_to_viewport({}, {}) == ({}, {}, {})", cx, cy, got.x, got.y, got.z)!;
		assert(vec3::eq(got, want));
	};
};

// An iterator over the pixels in the canvas and corresponding points in the viewport, going
// left-to-right, then top-to-bottom.
export type iterator = struct {
	v: *viewport,
	x: int,
	y: int,
};

// A point in the canvas / the viewport.
export type point = struct {
	cx: int,
	cy: int,
	on_viewport: vec3::vec3,
};

// Returns a new iterator. Call [[next]] in a loop to get the pixels on the canvas and corresponding
// points in the viewport.
export fn iterate(v: *viewport) iterator = {
	return iterator{v=v, x=0, y=v.c.height: int - 1};
};

// Returns the next point on the viewport.
export fn next(i: *iterator) (point | void) = {
	if (i.y < 0) {
		return;
	};
	const p = point{
		cx=i.x,
		cy=i.y,
		on_viewport=canvas_to_viewport(i.v, i.x, i.y),
	};
	i.x += 1;
	if (i.x >= i.v.c.width: int) {
		i.x = 0;
		i.y -= 1;
	};
	return p;
};

@test fn iterator() void = {
	// create 3x2 canvas, viewport, iterator
	const c = canvas::new(3, 2);
	defer canvas::finish(&c);
	const v = new(&c, 6.0, 2.5);
	const it = iterate(&v);

	// check the first one
	const p = next(&it) as point;
	const v = p.on_viewport;
	fmt::printfln("iterator yielded: {}, {}, ({}, {}, {})", p.cx, p.cy, v.x, v.y, v.z)!;
	assert(p.cx == 0);
	assert(p.cy == 1);
	assert(vec3::eq(v, vec3::new(-2.0, 1.0, 2.5)));

	// skip the next four
	assert(next(&it) is point);
	assert(next(&it) is point);
	assert(next(&it) is point);
	assert(next(&it) is point);

	// check the last one
	const p = next(&it) as point;
	const v = p.on_viewport;
	fmt::printfln("iterator yielded: {}, {}, ({}, {}, {})", p.cx, p.cy, v.x, v.y, v.z)!;
	assert(p.cx == 2);
	assert(p.cy == 0);
	assert(vec3::eq(v, vec3::new(2.0, -1.0, 2.5)));

	// make sure the iterator is done
	assert(next(&it) is void);
};
