/* Slow Hull Pseudo
slowHull(P)
	foreach p0 in P do
		foreach p1 in {P-p0} do
			foreach p2 in {P-p0-p1} do 1
				foreach p3 in {P-p0-p1-p2} do
					if p3 is contained within Triangle(p0,p1,p2) then
						mark p3 as internal 2

	create array A with all non-internal points in P
	determine leftmost point, left, in A
	sort A by angle formed with vertical line through left 3
	return A
*/
/*
	Points p0, p1, p2 form a triangle.
	Points not marks as internal are on convex hull.
	These angles (in degrees) range from -90 to 90.
*/
#[derive(Debug, Clone, Copy)]
struct Triangle {
	x: f32,
	y: f32,
	h: f32,
}

impl Triangle {
	fn connect(&self, dx:f32, dy:f32, step:f32) -> u32 { //magnitude vector connect
		dx = self.x;
		dy = self.y;

		let dx = (x2 - x1);
		let dy = (y2 - y1);

		if (dx.abs()>=dy.abs())
			step = abs(dx);
		else
			step = abs(dy);

		for (let i=0; i <= step) {
			x += dx;
			y += dy;
			i++;
			if (i>=step)
				break;
		}//digital differential analyzer
	}

	fn new(x, y, h: f32) -> Triangle {
		let x = &mut Triangle::x;
		let y = &mut Triangle::y;
		let h = &mut Triangle::h;

		return x, y, z;
	}

	fn from(x: f32, y: f32, h: f32) -> Triangle {
		Triangle {
			x: x,
			y: y,
			h: h,
		}
	}

	fn l(L: f32) -> f32 {
		let l: [f32; 1] = [0; 1];
		return L = l[1]
	}
}
//TODO(PIP): we need a Point-in-Polygon algorithm implementation
fn slowhull(P: f32) -> f32 {
	let mut L[f32; 1] = [0; 1];
	L = Triangle::l().L;

	#[inline]
	let mut P = Triangle()::from(L);
	#[inline]
	let p3 = Triangle::new(p0, p1, p2);

	for p0, p1, p2, p3 in L.iter() {
		let p1 = P-p0;
		let p2 = P-p0-p1;
		let p3 = P-p0-p1-p2;

		if p3 in P
		{
			#[inline]
			let p3 = Triangle::l(p0, p1, p2);
		}
	}//for p0 in p
}

struct convexhull {
	P: f32,
}

impl convexhull {
	fn merge(x, y: f32) -> u32 {
		let px = x;
		let py = y;

		return merge(px, py);
	}

	fn cvexhullk(&self) {//convexhull with nearest-k neighbours
		let P = self.P;
		if (P.len()==1)
			return P;
	
		halfsectx = cvexhullk(P[0; P.len()/2]);
		halfsecty = cvexhullk(P[P.len()/2;]);

		return merge(halfsect_x, halfsect_y);
	}
}