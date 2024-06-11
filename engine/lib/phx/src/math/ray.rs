use super::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ray {
    pub p: Position,
    pub dir: Vec3,
    pub tMin: f32,
    pub tMax: f32,
}

#[no_mangle]
pub extern "C" fn Ray_GetPoint(this: &Ray, t: f32, out: &mut Position) {
    *out = Position::from_dvec(this.p.v + (this.dir * t).as_dvec3());
}

#[no_mangle]
pub extern "C" fn Ray_IntersectPlane(this: &Ray, plane: &Plane, pHit: &mut Position) -> bool {
    Intersect_RayPlane(this, plane, pHit)
}

#[no_mangle]
pub unsafe extern "C" fn Ray_IntersectTriangle_Barycentric(
    this: &Ray,
    tri: &Triangle,
    tEpsilon: f32,
    tHit: &mut f32,
) -> bool {
    Intersect_RayTriangle_Barycentric(this, tri, tEpsilon, tHit)
}

#[no_mangle]
pub unsafe extern "C" fn Ray_IntersectTriangle_Moller1(
    this: &Ray,
    tri: &Triangle,
    tHit: &mut f32,
) -> bool {
    Intersect_RayTriangle_Moller1(this, tri, tHit)
}

#[no_mangle]
pub unsafe extern "C" fn Ray_IntersectTriangle_Moller2(
    this: &Ray,
    tri: &Triangle,
    tHit: &mut f32,
) -> bool {
    Intersect_RayTriangle_Moller2(this, tri, tHit)
}

#[no_mangle]
pub extern "C" fn Ray_ToLineSegment(this: &Ray, lineSegment: &mut LineSegment) {
    let mut p0 = Position::ZERO;
    let mut p1 = Position::ZERO;
    Ray_GetPoint(this, this.tMin, &mut p0);
    Ray_GetPoint(this, this.tMax, &mut p1);
    lineSegment.p0 = p0.as_vec3();
    lineSegment.p1 = p1.as_vec3();
}

#[no_mangle]
pub extern "C" fn Ray_FromLineSegment(lineSegment: &LineSegment, this: &mut Ray) {
    LineSegment_ToRay(lineSegment, this);
}
