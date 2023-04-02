use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::*;
use crate::Metric::*;
use crate::GL::gl;
use libc;

static mut alphaStack: [f32; 16] = [0.; 16];

static mut alphaIndex: i32 = -1;

static mut color: Vec4 = Vec4::ONE;

#[no_mangle]
pub unsafe extern "C" fn Draw_PushAlpha(a: f32) {
    if alphaIndex + 1 >= 16 {
        CFatal!("Draw_PushAlpha: Maximum alpha stack depth exceeded");
    }
    let prevAlpha: f32 = if alphaIndex >= 0 {
        alphaStack[alphaIndex as usize]
    } else {
        1.0f32
    };
    let alpha: f32 = a * prevAlpha;
    alphaIndex += 1;
    alphaStack[alphaIndex as usize] = alpha;
    gl::Color4f(color.x, color.y, color.z, color.w * alpha);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_PopAlpha() {
    if alphaIndex < 0 {
        CFatal!("Draw_PopAlpha Attempting to pop an empty alpha stack");
    }
    alphaIndex -= 1;
    let alpha: f32 = if alphaIndex >= 0 {
        alphaStack[alphaIndex as usize]
    } else {
        1.0f32
    };
    gl::Color4f(color.x, color.y, color.z, color.w * alpha);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Axes(
    pos: *const Vec3,
    x: *const Vec3,
    y: *const Vec3,
    z: *const Vec3,
    scale: f32,
    _alpha: f32,
) {
    let left: Vec3 = *pos + (*x) * scale;
    let up: Vec3 = *pos + (*y) * scale;
    let forward: Vec3 = *pos + (*z) * scale;
    gl::Begin(gl::LINES);
    gl::Color4f(1.0f32, 0.25f32, 0.25f32, _alpha);
    gl::Vertex3f((*pos).x, (*pos).y, (*pos).z);
    gl::Vertex3f(left.x, left.y, left.z);
    gl::Color4f(0.25f32, 1.0f32, 0.25f32, _alpha);
    gl::Vertex3f((*pos).x, (*pos).y, (*pos).z);
    gl::Vertex3f(up.x, up.y, up.z);
    gl::Color4f(0.25f32, 0.25f32, 1.0f32, _alpha);
    gl::Vertex3f((*pos).x, (*pos).y, (*pos).z);
    gl::Vertex3f(forward.x, forward.y, forward.z);
    gl::End();
    gl::Begin(gl::POINTS);
    gl::Color4f(1.0f32, 1.0f32, 1.0f32, _alpha);
    gl::Vertex3f((*pos).x, (*pos).y, (*pos).z);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Border(s: f32, x: f32, y: f32, w: f32, h: f32) {
    Draw_Rect(x, y, w, s);
    Draw_Rect(x, y + h - s, w, s);
    Draw_Rect(x, y + s, s, h - 2.0f32 * s);
    Draw_Rect(x + w - s, y + s, s, h - 2.0f32 * s);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Box3(this: *const Box3) {
    Metric_AddDrawImm(6, 12, 24);
    gl::Begin(gl::QUADS);
    gl::Vertex3f((*this).lower.x, (*this).lower.y, (*this).lower.z);
    gl::Vertex3f((*this).lower.x, (*this).lower.y, (*this).upper.z);
    gl::Vertex3f((*this).lower.x, (*this).upper.y, (*this).upper.z);
    gl::Vertex3f((*this).lower.x, (*this).upper.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).lower.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).upper.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).upper.y, (*this).upper.z);
    gl::Vertex3f((*this).upper.x, (*this).lower.y, (*this).upper.z);
    gl::Vertex3f((*this).lower.x, (*this).lower.y, (*this).upper.z);
    gl::Vertex3f((*this).upper.x, (*this).lower.y, (*this).upper.z);
    gl::Vertex3f((*this).upper.x, (*this).upper.y, (*this).upper.z);
    gl::Vertex3f((*this).lower.x, (*this).upper.y, (*this).upper.z);
    gl::Vertex3f((*this).lower.x, (*this).lower.y, (*this).lower.z);
    gl::Vertex3f((*this).lower.x, (*this).upper.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).upper.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).lower.y, (*this).lower.z);
    gl::Vertex3f((*this).lower.x, (*this).upper.y, (*this).lower.z);
    gl::Vertex3f((*this).lower.x, (*this).upper.y, (*this).upper.z);
    gl::Vertex3f((*this).upper.x, (*this).upper.y, (*this).upper.z);
    gl::Vertex3f((*this).upper.x, (*this).upper.y, (*this).lower.z);
    gl::Vertex3f((*this).lower.x, (*this).lower.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).lower.y, (*this).lower.z);
    gl::Vertex3f((*this).upper.x, (*this).lower.y, (*this).upper.z);
    gl::Vertex3f((*this).lower.x, (*this).lower.y, (*this).upper.z);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Clear(r: f32, g: f32, b: f32, a: f32) {
    let status: i32 = gl::CheckFramebufferStatus(gl::FRAMEBUFFER) as i32;
    if status != gl::FRAMEBUFFER_COMPLETE as i32 {
        CWarn!("Framebuffer is incomplete, skipping clear: %d", status,);
    } else {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    };
}

#[no_mangle]
pub unsafe extern "C" fn Draw_ClearDepth(d: f32) {
    gl::ClearDepth(d as f64);
    gl::Clear(gl::DEPTH_BUFFER_BIT);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Color(r: f32, g: f32, b: f32, a: f32) {
    let alpha: f32 = if alphaIndex >= 0 {
        alphaStack[alphaIndex as usize]
    } else {
        1.0f32
    };
    color = Vec4::new(r, g, b, a);
    gl::Color4f(r, g, b, a * alpha);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Flush() {
    Metric_Inc(0x6);
    gl::Finish();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Line(x1: f32, y1: f32, x2: f32, y2: f32) {
    gl::Begin(gl::LINES);
    gl::Vertex2f(x1, y1);
    gl::Vertex2f(x2, y2);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Line3(p1: *const Vec3, p2: *const Vec3) {
    gl::Begin(gl::LINES);
    gl::Vertex3f((*p1).x, (*p1).y, (*p1).z);
    gl::Vertex3f((*p2).x, (*p2).y, (*p2).z);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_LineWidth(width: f32) {
    gl::LineWidth(width);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Plane(p: *const Vec3, n: *const Vec3, scale: f32) {
    let mut e1: Vec3 = if f64::abs((*n).x as f64) < 0.7f64 {
        Vec3::X
    } else {
        Vec3::Y
    };
    e1 = Vec3_Reject(e1, *n).normalize();
    let e2: Vec3 = Vec3::cross(*n, e1);
    let p0: Vec3 = *p + (e1 * -scale) + (e2 * -scale);
    let p1: Vec3 = *p + (e1 * scale) + (e2 * -scale);
    let p2: Vec3 = *p + (e1 * scale) + (e2 * scale);
    let p3: Vec3 = *p + (e1 * -scale) + (e2 * scale);
    Metric_AddDrawImm(1, 2, 4);
    gl::Begin(gl::QUADS);
    gl::Vertex3f(p0.x, p0.y, p0.z);
    gl::Vertex3f(p1.x, p1.y, p1.z);
    gl::Vertex3f(p2.x, p2.y, p2.z);
    gl::Vertex3f(p3.x, p3.y, p3.z);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Point(x: f32, y: f32) {
    gl::Begin(gl::POINTS);
    gl::Vertex2f(x, y);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Point3(x: f32, y: f32, z: f32) {
    gl::Begin(gl::POINTS);
    gl::Vertex3f(x, y, z);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_PointSize(size: f32) {
    gl::PointSize(size);
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Poly(points: *const Vec2, count: i32) {
    Metric_AddDrawImm(1, count - 2, count);
    gl::Begin(gl::POLYGON);
    let mut i: i32 = 0;
    while i < count {
        gl::Vertex2f(
            (*points.offset(i as isize)).x,
            (*points.offset(i as isize)).y,
        );
        i += 1;
    }
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Poly3(points: *const Vec3, count: i32) {
    Metric_AddDrawImm(1, count - 2, count);
    gl::Begin(gl::POLYGON);
    let mut i: i32 = 0;
    while i < count {
        gl::Vertex3f(
            (*points.offset(i as isize)).x,
            (*points.offset(i as isize)).y,
            (*points.offset(i as isize)).z,
        );
        i += 1;
    }
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Quad(
    p1: *const Vec2,
    p2: *const Vec2,
    p3: *const Vec2,
    p4: *const Vec2,
) {
    Metric_AddDrawImm(1, 2, 4);
    gl::Begin(gl::QUADS);
    gl::TexCoord2f(0.0f32, 0.0f32);
    gl::Vertex2f((*p1).x, (*p1).y);
    gl::TexCoord2f(0.0f32, 1.0f32);
    gl::Vertex2f((*p2).x, (*p2).y);
    gl::TexCoord2f(1.0f32, 1.0f32);
    gl::Vertex2f((*p3).x, (*p3).y);
    gl::TexCoord2f(1.0f32, 0.0f32);
    gl::Vertex2f((*p4).x, (*p4).y);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Quad3(
    p1: *const Vec3,
    p2: *const Vec3,
    p3: *const Vec3,
    p4: *const Vec3,
) {
    Metric_AddDrawImm(1, 2, 4);
    gl::Begin(gl::QUADS);
    gl::TexCoord2f(0.0f32, 0.0f32);
    gl::Vertex3f((*p1).x, (*p1).y, (*p1).z);
    gl::TexCoord2f(0.0f32, 1.0f32);
    gl::Vertex3f((*p2).x, (*p2).y, (*p2).z);
    gl::TexCoord2f(1.0f32, 1.0f32);
    gl::Vertex3f((*p3).x, (*p3).y, (*p3).z);
    gl::TexCoord2f(1.0f32, 0.0f32);
    gl::Vertex3f((*p4).x, (*p4).y, (*p4).z);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Rect(x1: f32, y1: f32, xs: f32, ys: f32) {
    let x2: f32 = x1 + xs;
    let y2: f32 = y1 + ys;
    Metric_AddDrawImm(1, 2, 4);
    gl::Begin(gl::QUADS);
    gl::TexCoord2f(0.0f32, 0.0f32);
    gl::Vertex2f(x1, y1);
    gl::TexCoord2f(0.0f32, 1.0f32);
    gl::Vertex2f(x1, y2);
    gl::TexCoord2f(1.0f32, 1.0f32);
    gl::Vertex2f(x2, y2);
    gl::TexCoord2f(1.0f32, 0.0f32);
    gl::Vertex2f(x2, y1);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_SmoothLines(enabled: bool) {
    if enabled {
        gl::Enable(gl::LINE_SMOOTH);
        gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
    } else {
        gl::Disable(gl::LINE_SMOOTH);
        gl::Hint(gl::LINE_SMOOTH_HINT, gl::FASTEST);
    };
}

#[no_mangle]
pub unsafe extern "C" fn Draw_SmoothPoints(enabled: bool) {
    if enabled {
        gl::Enable(gl::POINT_SMOOTH);
        gl::Hint(gl::POINT_SMOOTH_HINT, gl::NICEST);
    } else {
        gl::Disable(gl::POINT_SMOOTH);
        gl::Hint(gl::POINT_SMOOTH_HINT, gl::FASTEST);
    };
}

#[inline]
extern "C" fn Spherical(r: f32, yaw: f32, pitch: f32) -> Vec3 {
    Vec3::new(
        (r as f64 * f64::sin(pitch as f64) * f64::cos(yaw as f64)) as f32,
        (r as f64 * f64::cos(pitch as f64)) as f32,
        (r as f64 * f64::sin(pitch as f64) * f64::sin(yaw as f64)) as f32,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Sphere(p: *const Vec3, r: f32) {
    let res: usize = 7;
    let fRes: f32 = res as f32;

    // First Row
    Metric_AddDrawImm(res as i32, res as i32, res.wrapping_mul(3) as i32);
    gl::Begin(gl::TRIANGLES);
    let mut lastTheta: f32 = res.wrapping_sub(1) as f32 / fRes * std::f32::consts::TAU;
    let phi: f32 = 1.0f32 / fRes * std::f32::consts::PI;
    let tc: Vec3 = *p + Spherical(r, 0.0f32, 0.0f32);
    let mut iTheta: usize = 0;
    while iTheta < res {
        let theta: f32 = iTheta as f32 / fRes * std::f32::consts::TAU;
        let br: Vec3 = *p + Spherical(r, lastTheta, phi);
        let bl: Vec3 = *p + Spherical(r, theta, phi);
        gl::Vertex3f(br.x, br.y, br.z);
        gl::Vertex3f(tc.x, tc.y, tc.z);
        gl::Vertex3f(bl.x, bl.y, bl.z);
        lastTheta = theta;
        iTheta = iTheta.wrapping_add(1);
    }
    gl::End();

    // Middle Rows
    Metric_AddDrawImm(
        res.wrapping_sub(2) as i32,
        2_usize.wrapping_mul(res.wrapping_sub(2)) as i32,
        4_usize.wrapping_mul(res.wrapping_sub(2)) as i32,
    );
    gl::Begin(gl::QUADS);
    let mut lastPhi: f32 = 1.0f32 / fRes * std::f32::consts::PI;
    let mut lastTheta_0: f32 = res.wrapping_sub(1) as f32 / fRes * std::f32::consts::TAU;
    let mut iPhi: usize = 2;
    while iPhi < res {
        let phi_0: f32 = iPhi as f32 / fRes * std::f32::consts::PI;
        let mut iTheta_0: usize = 0;
        while iTheta_0 < res {
            let theta_0: f32 = iTheta_0 as f32 / fRes * std::f32::consts::TAU;
            let br_0: Vec3 = *p + Spherical(r, lastTheta_0, phi_0);
            let tr: Vec3 = *p + Spherical(r, lastTheta_0, lastPhi);
            let tl: Vec3 = *p + Spherical(r, theta_0, lastPhi);
            let bl_0: Vec3 = *p + Spherical(r, theta_0, phi_0);
            gl::Vertex3f(br_0.x, br_0.y, br_0.z);
            gl::Vertex3f(tr.x, tr.y, tr.z);
            gl::Vertex3f(tl.x, tl.y, tl.z);
            gl::Vertex3f(bl_0.x, bl_0.y, bl_0.z);
            lastTheta_0 = theta_0;
            iTheta_0 = iTheta_0.wrapping_add(1);
        }
        lastPhi = phi_0;
        iPhi = iPhi.wrapping_add(1);
    }
    gl::End();

    // Bottom Row
    Metric_AddDrawImm(res as i32, res as i32, res.wrapping_mul(3) as i32);
    gl::Begin(gl::TRIANGLES);
    let mut lastTheta_1: f32 = res.wrapping_sub(1) as f32 / fRes * std::f32::consts::TAU;
    let phi_1: f32 = res.wrapping_sub(1) as f32 / fRes * std::f32::consts::PI;
    let bc: Vec3 = *p + Spherical(r, 0.0f32, std::f32::consts::PI);
    let mut iTheta_1: usize = 0;
    while iTheta_1 < res {
        let theta_1: f32 = iTheta_1 as f32 / fRes * std::f32::consts::TAU;
        let tr_0: Vec3 = *p + Spherical(r, lastTheta_1, phi_1);
        let tl_0: Vec3 = *p + Spherical(r, theta_1, phi_1);
        gl::Vertex3f(tr_0.x, tr_0.y, tr_0.z);
        gl::Vertex3f(tl_0.x, tl_0.y, tl_0.z);
        gl::Vertex3f(bc.x, bc.y, bc.z);
        lastTheta_1 = theta_1;
        iTheta_1 = iTheta_1.wrapping_add(1);
    }
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Tri(v1: *const Vec2, v2: *const Vec2, v3: *const Vec2) {
    Metric_AddDrawImm(1, 1, 3);
    gl::Begin(gl::TRIANGLES);
    gl::TexCoord2f(0.0f32, 0.0f32);
    gl::Vertex2f((*v1).x, (*v1).y);
    gl::TexCoord2f(0.0f32, 1.0f32);
    gl::Vertex2f((*v2).x, (*v2).y);
    gl::TexCoord2f(1.0f32, 1.0f32);
    gl::Vertex2f((*v3).x, (*v3).y);
    gl::End();
}

#[no_mangle]
pub unsafe extern "C" fn Draw_Tri3(v1: *const Vec3, v2: *const Vec3, v3: *const Vec3) {
    Metric_AddDrawImm(1, 1, 3);
    gl::Begin(gl::TRIANGLES);
    gl::TexCoord2f(0.0f32, 0.0f32);
    gl::Vertex3f((*v1).x, (*v1).y, (*v1).z);
    gl::TexCoord2f(0.0f32, 1.0f32);
    gl::Vertex3f((*v2).x, (*v2).y, (*v2).z);
    gl::TexCoord2f(1.0f32, 1.0f32);
    gl::Vertex3f((*v3).x, (*v3).y, (*v3).z);
    gl::End();
}
