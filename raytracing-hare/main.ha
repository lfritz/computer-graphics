use fmt;
use fs;
use io;
use math;
use os;

use canvas;
use color;
use vec3;

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
fn intersect_new(r: *ray, s: *sphere, kmin: f64, kmax: f64) (f64 | void) = {
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

@test fn intersect_new() void = {
	const s = sphere{
		center=vec3::new(0.0, 0.0, 2.0),
		radius=1.0,
		color=color::BLACK,
	};

	// ray doesn't intersect the sphere
	const r = ray{o=vec3::new(0.0, 1.5, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(intersect_new(&r, &s, 0.0, 8.0) is void);

	// ray intersects the sphere at (0.0, 1.0, 2.0)
	const r = ray{o=vec3::new(0.0, 1.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(intersect_new(&r, &s, 0.0, 1.5) is void);
	assert(intersect_new(&r, &s, 2.5, 8.0) is void);
	assert(math::isclose(2.0, intersect_new(&r, &s, 0.0, 8.0) as f64));

	// ray intersects the sphere at (0.0, 0.0, 1.0) and (0.0, 0.0, 3.0)
	const r = ray{o=vec3::new(0.0, 0.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(intersect_new(&r, &s, 0.0, 0.5) is void);
	assert(intersect_new(&r, &s, 3.5, 8.0) is void);
	assert(math::isclose(1.0, intersect_new(&r, &s, 0.0, 2.5) as f64));
	assert(math::isclose(3.0, intersect_new(&r, &s, 1.5, 8.0) as f64));
};

type scene = struct {
	spheres: []*sphere,
	background: color::color,
};

fn trace_ray(scene: *scene, r: *ray, tmin: f64, tmax: f64) color::color = {
	let closest_sphere: nullable *sphere = null;

	for (let i: size = 0; i < len(scene.spheres); i += 1) {
		const sp = scene.spheres[i];
		match (intersect_new(r, sp, tmin, tmax)) {
		case void => continue;
		case let t: f64 =>
			tmax = t;
			closest_sphere = sp;
		};
	};

	match (closest_sphere) {
		case null            => return scene.background;
		case let sp: *sphere => return sp.color;
	};
};

@test fn trace_ray() void = {
	const red = color::new(255, 0, 0);
	const green = color::new(0, 255, 0);
	const white = color::WHITE;

	const s1 = alloc(sphere{
		center=vec3::new(0.0, 0.0, 2.0),
		radius=1.0,
		color=red,
	});
	defer free(s1);

	const s2 = alloc(sphere{
		center=vec3::new(0.0, 4.0, 2.0),
		radius=2.0,
		color=green,
	});
	defer free(s2);

	const scene = scene{spheres=[s1, s2], background=white};

	// ray passes above the red sphere and below the green sphere
	const r = ray{o=vec3::new(0.0, 1.5, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(color::eq(trace_ray(&scene, &r, 0.0, 8.0), white));

	// ray intersects the red sphere in one point
	const r = ray{o=vec3::new(0.0, 1.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(color::eq(trace_ray(&scene, &r, 0.0, 8.0), red));

	// ray goes through red sphere
	const r = ray{o=vec3::new(0.0, 0.0, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	assert(color::eq(trace_ray(&scene, &r, 0.0, 0.5), white));
	assert(color::eq(trace_ray(&scene, &r, 4.0, 8.0), white));
	assert(color::eq(trace_ray(&scene, &r, 2.0, 4.0), red));
	assert(color::eq(trace_ray(&scene, &r, 0.0, 2.0), red));
	assert(color::eq(trace_ray(&scene, &r, 1.0, 4.0), red));

	// ray goes through the red sphere, then the green one
	const r = ray{o=vec3::new(0.0, -2.0, 2.0), d=vec3::new(0.0, 1.0, 0.0)};
	assert(color::eq(trace_ray(&scene, &r, 0.0, 10.0), red));
	assert(color::eq(trace_ray(&scene, &r, 2.0, 10.0), red));
	assert(color::eq(trace_ray(&scene, &r, 4.0, 10.0), green));
	assert(color::eq(trace_ray(&scene, &r, 6.0, 10.0), green));
	assert(color::eq(trace_ray(&scene, &r, 9.0, 10.0), white));
};

export fn main() void = {
	const s1 = alloc(sphere{
		center=vec3::new(0.0, -1.0, -3.0),
		radius=1.0,
		color=color::new(255, 0, 0),
	});
	defer free(s1);

	const s2 = alloc(sphere{
		center=vec3::new(2.0, 0.0, 4.0),
		radius=1.0,
		color=color::new(0, 0, 255),
	});
	defer free(s2);

	const s3 = alloc(sphere{
		center=vec3::new(-2.0, 0.0, 4.0),
		radius=1.0,
		color=color::new(0, 255, 0),
	});
	defer free(s3);

	const scene = scene{
		spheres=[s1, s2, s3],
		background=color::WHITE,
	};

	let c = canvas::new(640, 480);
	defer canvas::finish(&c);

	const r = ray{o=vec3::new(0.0, 1.5, 0.0), d=vec3::new(0.0, 0.0, 1.0)};
	trace_ray(&scene, &r, 0.0, 10.0);

	const mode = fs::mode::USER_RW | fs::mode::GROUP_R | fs::mode::OTHER_R | fs::mode::REG;
	const file = os::create("image.ppm", mode, fs::flag::WRONLY)!;
	defer io::close(file)!;
	canvas::write(file, &c)!;
};
