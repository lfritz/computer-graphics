use fmt;
use fs;
use io;
use math;
use os;

use canvas;
use color;
use vec3;
use viewport;

// Returns whether the slices' corresponding elements are within math::STANDARD_TOL of each other.
fn isclose_f64s(a: []f64, b: []f64) bool = {
	if (len(a) != len(b)) {
		return false;
	};
	for (let i: size = 0; i < len(a); i += 1) {
		if (!math::isclose(a[i], b[i])) {
			return false;
		};
	};
	return true;
};

@test fn isclose_f64s() void = {
	const small = 0.5 * math::STANDARD_TOL;
	const large = 2.0 * math::STANDARD_TOL;
	const a = [1.0, 2.0, -3.0];
	const b = [1.0, 2.0];            assert(!isclose_f64s(a, b));
	const b = [1.0, 2.0, -3.0, 4.0]; assert(!isclose_f64s(a, b));
	const b = a;                     assert(isclose_f64s(a, b));
	const b = a; b[0] += small;      assert(isclose_f64s(a, b));
	const b = a; b[1] -= small;      assert(isclose_f64s(a, b));
	const b = a; b[2] += small;      assert(isclose_f64s(a, b));
	const b = a; b[0] += large;      assert(!isclose_f64s(a, b));
	const b = a; b[1] -= large;      assert(!isclose_f64s(a, b));
	const b = a; b[2] += large;      assert(!isclose_f64s(a, b));
};

// A ray, defined by its origin an direction.
type ray = struct {
	o: vec3::vec3, // origin
	d: vec3::vec3, // direction, cannot be zero
};

// Returns a point on the ray, following the formula o + k*d.
fn rayat(r: *ray, k: f64) vec3::vec3 = {
	return vec3::add(r.o, vec3::mul(k, r.d));
};

@test fn rayat() void = {
	const r = ray{o=vec3::new(1.0, 2.0, 3.0), d=vec3::new(0.5, 1.5, -0.5)};
	assert(vec3::eq(rayat(&r, 0.0), vec3::new(1.0, 2.0, 3.0)));
	assert(vec3::eq(rayat(&r, 1.0), vec3::new(1.5, 3.5, 2.5)));
	assert(vec3::eq(rayat(&r, -2.0), vec3::new(0.0, -1.0, 4.0)));
};

// A sphere, defined by its center and radius.
type sphere = struct {
	center: vec3::vec3,
	radius: f64,
	color: color::color,
};

fn smallest_within_range(k1: f64, k2: f64, kmin: f64, kmax: f64) (f64 | void) = {
	const k1_valid = kmin <= k1 && k1 <= kmax;
	const k2_valid = kmin <= k2 && k2 <= kmax;
	if (!k1_valid && !k2_valid) { return; };
	if ( k1_valid && !k2_valid) { return k1; };
	if (!k1_valid &&  k2_valid) { return k2; };
	if (k1 < k2) {
		return k1;
	};
	return k2;
};

@test fn smallest_within_range() void = {
	assert(smallest_within_range(1.0, 2.0, 3.0, 4.0) is void);
	assert(smallest_within_range(2.0, 1.0, 3.0, 4.0) is void);
	assert(smallest_within_range(1.0, 2.0, 0.0, 1.5) as f64 == 1.0);
	assert(smallest_within_range(2.0, 1.0, 0.0, 1.5) as f64 == 1.0);
	assert(smallest_within_range(1.0, 2.0, 1.5, 2.5) as f64 == 2.0);
	assert(smallest_within_range(2.0, 1.0, 1.5, 2.5) as f64 == 2.0);
	assert(smallest_within_range(1.0, 2.0, 0.0, 3.0) as f64 == 1.0);
	assert(smallest_within_range(2.0, 1.0, 0.0, 3.0) as f64 == 1.0);
};

// Finds the points where the ray and sphere intersect. It returns the smallest value k for which
// r.o + k * r.d is on the sphere's surface and kmin <= k <= kmax.
fn intersect(r: *ray, s: *sphere, kmin: f64, kmax: f64) (f64 | void) = {
	const co = vec3::sub(r.o, s.center);
	const a = vec3::dot(r.d, r.d);
	const b = 2.0 * vec3::dot(co, r.d);
	const c = vec3::dot(co, co) - s.radius*s.radius;
	const d = b*b - 4.0*a*c;
	const points: []vec3::vec3 = [];
	if (d < 0.0) {
		return;
	};
	const k1 = ( - b + math::sqrtf64(d)) / (2.0*a);
	const k2 = ( - b - math::sqrtf64(d)) / (2.0*a);
	return smallest_within_range(k1, k2, kmin, kmax);
};

@test fn intersect() void = {
	const s = sphere{
		center=vec3::new(0.0, 0.0, 2.0),
		radius=1.0,
		color=color::BLACK,
	};

	// ray doesn't intersect the sphere
	const r = ray{o=vec3::new(0.0, 1.5, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(intersect(&r, &s, 0.0, 8.0) is void);

	// ray intersects the sphere at (0.0, 1.0, 2.0)
	const r = ray{o=vec3::new(0.0, 1.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(intersect(&r, &s, 0.0, 1.5) is void);
	assert(intersect(&r, &s, 2.5, 8.0) is void);
	assert(math::isclose(2.0, intersect(&r, &s, 0.0, 8.0) as f64));

	// ray intersects the sphere at (0.0, 0.0, 1.0) and (0.0, 0.0, 3.0)
	const r = ray{o=vec3::new(0.0, 0.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(intersect(&r, &s, 0.0, 0.5) is void);
	assert(intersect(&r, &s, 3.5, 8.0) is void);
	assert(math::isclose(1.0, intersect(&r, &s, 0.0, 2.5) as f64));
	assert(math::isclose(3.0, intersect(&r, &s, 1.5, 8.0) as f64));
};

type scene = struct {
	spheres: []*sphere,
	lights: lights,
	background: color::color,
};

type hit = struct {
	sphere: *sphere,
	point: vec3::vec3,
};

fn trace_ray(spheres: []*sphere, r: *ray, tmin: f64, tmax: f64) (hit | void) = {
	let closest_sphere: nullable *sphere = null;

	for (let i: size = 0; i < len(spheres); i += 1) {
		const sp = spheres[i];
		match (intersect(r, sp, tmin, tmax)) {
		case void => continue;
		case let t: f64 =>
			tmax = t;
			closest_sphere = sp;
		};
	};

	match (closest_sphere) {
		case null            => return;
		case let sp: *sphere => return hit{sphere=sp, point=rayat(r, tmax)};
	};
};

@test fn trace_ray() void = {
	const s1 = alloc(sphere{
		center=vec3::new(0.0, 0.0, 2.0),
		radius=1.0,
		color=color::new(255, 0, 0),
	});
	defer free(s1);

	const s2 = alloc(sphere{
		center=vec3::new(0.0, 4.0, 2.0),
		radius=2.0,
		color=color::new(0, 255, 0),
	});
	defer free(s2);

	const spheres = [s1, s2];

	// r2 doesn't intersect s1 or s2
	const r1 = ray{o=vec3::new(0.0, 1.5, 0.0), d=vec3::new(0.0, 0.0, 1.0)};

	// r2 intersects s1 in exactly one point
	const r2 = ray{o=vec3::new(0.0, 1.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};

	// r3 goes passes through the center of s1
	const r3 = ray{o=vec3::new(0.0, 0.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};

	// r4 passes through s1, then through s2
	const r4 = ray{o=vec3::new(0.0, -2.0, 2.0), d=vec3::new(0.0, 1.0, 0.0)};

	// test cases where trace_ray should return void
	const cases: [](*ray, f64, f64) = [
		(&r1, 0.0, 8.0),
		(&r2, 0.0, 0.5),
		(&r2, 3.5, 8.0),
		(&r4, 9.0, 10.0),
	];
	for (let i: size = 0; i < len(cases); i += 1) {
		const (r, tmin, tmax) = cases[i];
		assert(trace_ray(spheres, r, tmin, tmax) is void);
	};

	// test cases where trace_ray should return a hit
	const cases: [](*ray, f64, f64, hit) = [
		(&r2, 0.0,  8.0, hit{sphere=s1, point=vec3::new(0.0,  1.0, 2.0)}),
		(&r3, 0.0,  2.0, hit{sphere=s1, point=vec3::new(0.0,  0.0, 1.0)}),
		(&r3, 2.0,  4.0, hit{sphere=s1, point=vec3::new(0.0,  0.0, 3.0)}),
		(&r4, 0.0, 10.0, hit{sphere=s1, point=vec3::new(0.0, -1.0, 2.0)}),
		(&r4, 2.0, 10.0, hit{sphere=s1, point=vec3::new(0.0,  1.0, 2.0)}),
		(&r4, 4.0, 10.0, hit{sphere=s2, point=vec3::new(0.0,  2.0, 2.0)}),
		(&r4, 6.0, 10.0, hit{sphere=s2, point=vec3::new(0.0,  6.0, 2.0)}),
	];
	for (let i: size = 0; i < len(cases); i += 1) {
		const (r, tmin, tmax, want) = cases[i];
		const got = trace_ray(spheres, r, tmin, tmax) as hit;
		assert(got.sphere == want.sphere);
		assert(vec3::isclose(got.point, want.point));
	};
};

type directional_light = struct {
	l: vec3::vec3,  // direction to the light source
	i: f64,         // intensity
};

type point_light = struct {
	p: vec3::vec3,  // location of the light source
	i: f64,         // intensity
};

type lights = struct {
	ambient: f64,
	directional: []directional_light,
	point: []point_light,
};

fn light_intensity(n: vec3::vec3, l: vec3::vec3, i: f64) f64 = {
	const n_dot_l = vec3::dot(n, l);
	if (n_dot_l <= 0.0) {
		return 0.0;
	};
	return i * n_dot_l/(vec3::length(n) * vec3::length(l));
};

@test fn light_intensity() void = {
	const l = vec3::new(0.0, 2.0, 0.0);
	const i = 0.6;

	// n and l have the same direction: full illumination
	assert(math::isclose(light_intensity(vec3::new(0.0, 1.0, 0.0), l, i), i));

	// 90 degree angle between n and l: no illumination
	assert(math::isclose(light_intensity(vec3::new(1.0, 0.0, 0.0), l, i), 0.0));

	// surface is facing away from the light source: no illumination
	assert(math::isclose(light_intensity(vec3::new(0.0, -1.0, 0.0), l, i), 0.0));
	assert(math::isclose(light_intensity(vec3::new(1.0, -1.0, 0.0), l, i), 0.0));

	// 45 degree angle between n and l
	const attenuation = 1.0 / math::SQRT_2;  // attenuation at 45 degrees
	assert(math::isclose(light_intensity(vec3::new(1.0, 1.0, 0.0), l, i), i * attenuation));
	assert(math::isclose(light_intensity(vec3::new(0.0, 1.0, 1.0), l, i), i * attenuation));
};

fn compute_lighting(p: vec3::vec3, n: vec3::vec3, ls: *lights) f64 = {
	let result = ls.ambient;
	for (let i: size = 0; i < len(ls.directional); i += 1) {
		result += light_intensity(n, ls.directional[i].l, ls.directional[i].i);
	};
	for (let i: size = 0; i < len(ls.point); i += 1) {
		const l = vec3::sub(ls.point[i].p, p);
		result += light_intensity(n, l, ls.point[i].i);
	};
	return result;
};

@test fn compute_lighting() void = {
	const p = vec3::new(1.0, 2.0, 3.0);
	const up = vec3::new(0.0, 1.0, 0.0);
	const down = vec3::new(0.0, -1.0, 0.0);
	const above = vec3::new(1.0, 8.0, 3.0);
	const below = vec3::new(1.0, -8.0, 3.0);

	// just ambient light
	const ls = lights{
		ambient=0.1,
		directional=[],
		point=[],
	};
	assert(math::isclose(compute_lighting(p, up, &ls), 0.1));

	// one directional light
	const ls = lights{
		ambient=0.1,
		directional=[directional_light{l=up, i=0.2}],
		point=[],
	};
	assert(math::isclose(compute_lighting(p, up, &ls), 0.3));

	// one point light
	const ls = lights{
		ambient=0.1,
		directional=[],
		point=[point_light{p=above, i=0.3}],
	};
	assert(math::isclose(compute_lighting(p, up, &ls), 0.4));

	// one each
	const ls = lights{
		ambient=0.1,
		directional=[directional_light{l=up, i=0.2}],
		point=[point_light{p=above, i=0.3}],
	};
	assert(math::isclose(compute_lighting(p, up, &ls), 0.6));

	// three each
	const ls = lights{
		ambient=0.1,
		directional=[
			directional_light{l=up, i=0.05},
			directional_light{l=up, i=0.15},
			directional_light{l=down, i=0.15},
		],
		point=[
			point_light{p=above, i=0.25},
			point_light{p=above, i=0.35},
			point_light{p=below, i=0.35},
		],
	};
	assert(math::isclose(compute_lighting(p, up, &ls), 0.9));
};

fn raytrace_pixel(sc: *scene, p: vec3::vec3) color::color = {
	const origin = vec3::new(0.0, 0.0, 0.0);
	const r = ray{o=origin, d=vec3::sub(p, origin)};
	match (trace_ray(sc.spheres, &r, 1.0, math::INF)) {
		case void => return sc.background;
		case let h: hit => {
			const normal = vec3::sub(h.point, h.sphere.center);
			const intensity = compute_lighting(h.point, normal, &sc.lights);
			return color::mul(intensity, h.sphere.color);
		};
	};
};

@test fn raytrace_pixel() void = {
	const s1 = alloc(sphere{
		center=vec3::new(0.0, 0.0, 3.0),
		radius=1.0,
		color=color::new(0xa0, 0xc0, 0xe0),
	});
	defer free(s1);

	const sc = scene{
		spheres=[s1],
		lights=lights{
			ambient=0.5,
			directional=[],
			point=[],
		},
		background=color::BLACK,
	};

	// trace ray that won't hit the sphere
	const got = raytrace_pixel(&sc, vec3::new(0.0, 0.5, 1.0));
	assert(color::eq(got, color::BLACK));

	// trace ray that'll hit the sphere at 90 degree angle
	const got = raytrace_pixel(&sc, vec3::new(0.0, 0.0, 1.0));
	assert(color::eq(got, color::new(0x50, 0x60, 0x70)));
};

fn raytrace_image(c: *canvas::canvas, sc: *scene) void = {
	const v = viewport::new(c, 1.0, 1.0);
	let it = viewport::iterate(&v);
	for (true) {
		match (viewport::next(&it)) {
		case void => break;
		case let p: viewport::point =>
			const col = raytrace_pixel(sc, p.on_viewport);
			canvas::putpixel(c, p.cx, p.cy, col);
		};
	};
};

@test fn raytrace_image() void = {
	// XXX find a way to set up the scene in a helper function
	const s1 = alloc(sphere{
		center=vec3::new(0.0, 0.0, 6.0),
		radius=1.0,
		color=color::new(0xa0, 0xc0, 0xe0),
	});
	defer free(s1);

	const sc = scene{
		spheres=[s1],
		lights=lights{
			ambient=0.5,
			directional=[],
			point=[],
		},
		background=color::BLACK,
	};

	const got = canvas::new(5, 3);
	defer canvas::finish(&got);

	raytrace_image(&got, &sc);

	const want = canvas::new(5, 3);
	for (let y = 0; y < 3; y += 1) {
		for (let x = 0; x < 5; x += 1) {
			canvas::putpixel(&want, x, y, color::BLACK);
		};
	};
	canvas::putpixel(&want, 2, 1, color::new(0x50, 0x60, 0x70));

	assert(canvas::eq(&got, &want));
};

export fn main() void = {
	const s1 = alloc(sphere{
		center=vec3::new(0.0, -1.0, 6.0),
		radius=1.0,
		color=color::new(0xff, 0, 0),
	});
	defer free(s1);

	const s2 = alloc(sphere{
		center=vec3::new(2.0, 0.0, 4.0),
		radius=1.0,
		color=color::new(0, 0, 0xff),
	});
	defer free(s2);

	const s3 = alloc(sphere{
		center=vec3::new(-2.0, 0.0, 4.0),
		radius=1.0,
		color=color::new(0, 0xff, 0),
	});
	defer free(s3);

	const s4 = alloc(sphere{
		center=vec3::new(0.0, -5001.0, 0.0),
		radius=5000.0,
		color=color::new(0xff, 0xff, 0),
	});
	defer free(s4);

	const sc = scene{
		spheres=[s1, s2, s3, s4],
		lights=lights{
			ambient=0.2,
			directional=[
				directional_light{l=vec3::new(1.0, 4.0, 4.0), i=0.2},
			],
			point=[
				point_light{p=vec3::new(2.0, 2.0, 1.0), i=0.6},
			],
		},
		background=color::BLACK,
	};

	const c = canvas::new(640, 480);
	defer canvas::finish(&c);

	raytrace_image(&c, &sc);

	const mode = fs::mode::USER_RW | fs::mode::GROUP_R | fs::mode::OTHER_R | fs::mode::REG;
	const file = os::create("image.ppm", mode, fs::flag::WRONLY)!;
	defer io::close(file)!;
	canvas::write(file, &c)!;
};
