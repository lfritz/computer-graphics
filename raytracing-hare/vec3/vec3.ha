use math;

// A vector in 3-D space.
export type vec3 = struct {
	x: f64,
	y: f64,
	z: f64,
};

// Add two vectors.
export fn add(a: vec3, b: vec3) vec3 = {
	return vec3{
		x=a.x + b.x,
		y=a.y + b.y,
		z=a.z + b.z,
	};
};

@test fn add() void = {
	const a = vec3{x=1.0, y=2.0, z=3.0};
	const b = vec3{x=-2.0, y=0.5, z=3.0};
	const got = add(a, b);
	assert(got.x == -1.0);
	assert(got.y == 2.5);
	assert(got.z == 6.0);
};

// Subtract two vectors.
export fn sub(a: vec3, b: vec3) vec3 = {
	return vec3{
		x=a.x - b.x,
		y=a.y - b.y,
		z=a.z - b.z,
	};
};

@test fn sub() void = {
	const a = vec3{x=1.0, y=2.0, z=3.0};
	const b = vec3{x=-2.0, y=0.5, z=3.0};
	const got = sub(a, b);
	assert(got.x == 3.0);
	assert(got.y == 1.5);
	assert(got.z == 0.0);
};

// Multiply a vector by a constant.
export fn mul(k: f64, v: vec3) vec3 = {
	return vec3{
		x=k * v.x,
		y=k * v.y,
		z=k * v.z,
	};
};

@test fn mul() void = {
	const v = vec3{x=1.0, y=2.0, z=3.0};
	const got = mul(-2.0, v);
	assert(got.x == -2.0);
	assert(got.y == -4.0);
	assert(got.z == -6.0);
};

// Calculate the square of the vector's length.
export fn sqlen(v: vec3) f64 = {
	return v.x * v.x +
	       v.y * v.y +
	       v.z * v.z;
};

@test fn sqlen() void = {
	assert(sqlen(vec3{x=1.0, y=2.0, z=3.0}) == 14.0);
};

// Calculate the vector's length (Euclidian norm).
export fn length(v: vec3) f64 = {
	return math::sqrtf64(sqlen(v));
};

@test fn length() void = {
	assert(length(vec3{x=3.0, y=4.0, z=0.0}) == 5.0);
	assert(length(vec3{x=0.0, y=3.0, z=4.0}) == 5.0);
	assert(length(vec3{x=4.0, y=0.0, z=3.0}) == 5.0);
};

// Calculate the dot product of two vectors.
export fn dot(a: vec3, b: vec3) f64 = {
	return a.x * b.x +
	       a.y * b.y +
	       a.z * b.z;
};

@test fn dot() void = {
	const a = vec3{x=1.0, y=2.0, z=3.0};
	const b = vec3{x=-1.0, y=0.5, z=2.0};
	assert(dot(a, b) == 6.0);
	assert(dot(b, a) == 6.0);
};

// Calculate the cross product of two vectors.
export fn cross(a: vec3, b: vec3) vec3 = {
	return vec3{
		x=a.y*b.z - a.z*b.y,
		y=a.x*b.z - a.z*b.x,
		z=a.x*b.y - a.y*b.x,
	};
};

@test fn cross() void = {
	const a = vec3{x=1.0, y=2.0, z=3.0};
	const b = vec3{x=-1.0, y=0.5, z=2.0};
	const got = cross(a, b);
	assert(got.x == 4.0 - 1.5);
	assert(got.y == 2.0 + 3.0);
	assert(got.z == 0.5 + 2.0);
};
