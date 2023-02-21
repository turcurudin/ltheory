use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
use glam::Vec2;

extern "C" {
    pub type Mesh;
    pub type RNG;
    fn fabs(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn Mesh_GetVertexData(_: *mut Mesh) -> *mut Vertex;
    fn Mesh_GetIndexData(_: *mut Mesh) -> *mut libc::c_int;
    fn Mesh_GetIndexCount(_: *mut Mesh) -> libc::c_int;
    fn Intersect_SphereTriangle(
        _: *const Sphere,
        _: *const Triangle,
        pHit: *mut Vec3,
    ) -> bool;
    fn Intersect_RayTriangle_Moller1(
        _: *const Ray,
        _: *const Triangle,
        tHit: *mut f32,
    ) -> bool;
    fn Fatal(_: cstr, _: ...);
    fn Warn(_: cstr, _: ...);
    fn Plane_ClassifyPolygon(_: *mut Plane, _: *mut Polygon) -> PolygonClassification;
    fn Polygon_ToPlane(_: *mut Polygon, _: *mut Plane);
    fn Polygon_SplitSafe(
        _: *mut Polygon,
        splitPlane: Plane,
        back: *mut Polygon,
        front: *mut Polygon,
    );
    fn Polygon_ConvexToTriangles(
        _: *mut Polygon,
        triangles_capacity: *mut int32,
        triangles_size: *mut int32,
        triangles_data: *mut *mut Triangle,
    );
    fn Ray_GetPoint(_: *const Ray, t: f32, out: *mut Vec3);
    fn RNG_Create(seed: uint64) -> *mut RNG;
    fn RNG_Free(_: *mut RNG);
    fn RNG_Get32(_: *mut RNG) -> uint32;
    fn Draw_Sphere(p: *const Vec3, r: f32);
    fn Draw_PointSize(size: f32);
    fn Draw_Poly3(points: *const Vec3, count: libc::c_int);
    fn Draw_Point3(x: f32, y: f32, z: f32);
    fn Draw_Color(
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );
    fn Draw_Plane(p: *const Vec3, n: *const Vec3, scale: f32);
    fn Draw_Line3(p1: *const Vec3, p2: *const Vec3);
    fn RenderState_PushBlendMode(_: BlendMode);
    fn RenderState_PushCullFace(_: CullFace);
    fn RenderState_PushDepthTest(_: bool);
    fn RenderState_PushWireframe(_: bool);
    fn RenderState_PopBlendMode();
    fn RenderState_PopCullFace();
    fn RenderState_PopDepthTest();
    fn RenderState_PopWireframe();
}
pub type int32_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type cstr = *const libc::c_char;
pub type int32 = int32_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type uint64 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSP {
    pub rootNode: BSPNodeRef,
    pub emptyLeaf: BSPNodeRef,
    pub nodes_size: int32,
    pub nodes_capacity: int32,
    pub nodes_data: *mut BSPNode,
    pub triangles_size: int32,
    pub triangles_capacity: int32,
    pub triangles_data: *mut Triangle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPNode {
    pub plane: Plane,
    pub child: [BSPNodeRef; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPNodeRef {
    pub index: int32,
    pub triangleCount: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Plane {
    pub n: Vec3,
    pub d: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntersectSphereProfiling {
    pub nodes: int32,
    pub leaves: int32,
    pub triangles: int32,
    pub triangleTests_size: int32,
    pub triangleTests_capacity: int32,
    pub triangleTests_data: *mut TriangleTest,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriangleTest {
    pub triangle: *mut Triangle,
    pub hit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LineSegment {
    pub p0: Vec3,
    pub p1: Vec3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Polygon {
    pub vertices_size: int32,
    pub vertices_capacity: int32,
    pub vertices_data: *mut Vec3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ray {
    pub p: Vec3,
    pub dir: Vec3,
    pub tMin: f32,
    pub tMax: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sphere {
    pub p: Vec3,
    pub r: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub p: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
}
pub type BlendMode = int32;
pub type BSPNodeRel = uint8;
pub type CullFace = int32;
pub type PolygonClassification = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPBuild {
    pub rootNode: *mut BSPBuild_Node,
    pub rng: *mut RNG,
    pub nodeCount: int32,
    pub leafCount: int32,
    pub triangleCount: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPBuild_Node {
    pub plane: Plane,
    pub child: [*mut BSPBuild_Node; 2],
    pub polygons_size: int32,
    pub polygons_capacity: int32,
    pub polygons_data: *mut PolygonEx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonEx {
    pub vertices_size: int32,
    pub vertices_capacity: int32,
    pub vertices_data: *mut Vec3,
    pub flags: PolygonFlag,
}
pub type PolygonFlag = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPBuild_NodeData {
    pub polygons_size: int32,
    pub polygons_capacity: int32,
    pub polygons_data: *mut PolygonEx,
    pub validPolygonCount: int32,
    pub triangleCount: int32,
    pub depth: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DelayRay {
    pub nodeRef: BSPNodeRef,
    pub tMin: f32,
    pub tMax: f32,
    pub depth: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Delay {
    pub nodeRef: BSPNodeRef,
    pub depth: int32,
}

#[inline]
unsafe extern "C" fn Abs(mut t: f64) -> f64 {
    return fabs(t);
}
#[inline]
unsafe extern "C" fn Sqrtf(mut t: f32) -> f32 {
    return sqrt(t as f64) as f32;
}
#[inline]
unsafe extern "C" fn Lerp(
    mut a: f64,
    mut b: f64,
    mut t: f64,
) -> f64 {
    return a + t * (b - a);
}
#[inline]
unsafe extern "C" fn Max(
    mut a: f64,
    mut b: f64,
) -> f64 {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn Min(
    mut a: f64,
    mut b: f64,
) -> f64 {
    return if a < b { a } else { b };
}
#[no_mangle]
pub static mut BSPNodeRel_Parent: BSPNodeRel = 0 as libc::c_int as BSPNodeRel;
#[no_mangle]
pub static mut BSPNodeRel_Back: BSPNodeRel = 1 as libc::c_int as BSPNodeRel;
#[no_mangle]
pub static mut BSPNodeRel_Front: BSPNodeRel = 2 as libc::c_int as BSPNodeRel;
static mut BackIndex: int32 = 0 as libc::c_int;
static mut FrontIndex: int32 = 1 as libc::c_int;
static mut RootNodeIndex: int32 = 1 as libc::c_int;
static mut EmptyLeafIndex: int32 = 1 as libc::c_int;
#[no_mangle]
pub static mut rayStack_size: int32 = 0;
#[no_mangle]
pub static mut rayStack_capacity: int32 = 0;
#[no_mangle]
pub static mut rayStack_data: *mut DelayRay = 0 as *const DelayRay as *mut DelayRay;
#[no_mangle]
pub unsafe extern "C" fn BSP_IntersectRay(
    mut this: *mut BSP,
    mut _ray: *const Ray,
    mut tHit: *mut f32,
) -> bool {
    let mut ray: Ray = *_ray;
    *tHit = 3.40282347e+38f32;
    let mut nodeRef: BSPNodeRef = (*this).rootNode;
    let mut tEpsilon: f32 = (8.0f32 as f64 * 1e-4f64
        / ray.dir.length() as f64) as f32;
    let mut hit: bool = 0 as libc::c_int != 0;
    let mut depth: int32 = 0 as libc::c_int;
    let mut maxDepth: int32 = 0 as libc::c_int;
    loop {
        maxDepth = Max(depth as f64, maxDepth as f64) as int32;
        if nodeRef.index >= 0 as libc::c_int {
            let mut node: *mut BSPNode = ((*this).nodes_data)
                .offset(nodeRef.index as isize);
            let mut dist: f32 = Vec3::dot((*node).plane.n, ray.p)
                - (*node).plane.d;
            let mut denom: f32 = -Vec3::dot((*node).plane.n, ray.dir);
            let mut nearIndex: libc::c_int = (dist > 0.0f32)
                as libc::c_int;
            let mut earlyIndex: libc::c_int = nearIndex;
            if denom != 0.0f32 {
                let mut t: f32 = dist / denom;
                let mut planeBegin: f32 = t - tEpsilon;
                let mut planeEnd: f32 = t + tEpsilon;
                if !(planeBegin >= ray.tMax) {
                    if planeEnd <= ray.tMin {
                        earlyIndex = (t >= 0.0f32) as libc::c_int ^ nearIndex;
                    } else {
                        earlyIndex = (t < 0.0f32) as libc::c_int ^ nearIndex;
                        let mut min: f32 = Max(
                            planeBegin as f64,
                            ray.tMin as f64,
                        ) as f32;
                        let mut max: f32 = Min(
                            planeEnd as f64,
                            ray.tMax as f64,
                        ) as f32;
                        let mut d: DelayRay = {
                            let mut init = DelayRay {
                                nodeRef: (*node)
                                    .child[(1 as libc::c_int ^ earlyIndex) as usize],
                                tMin: min,
                                tMax: ray.tMax,
                                depth: depth,
                            };
                            init
                        };
                        if (rayStack_capacity == rayStack_size) as libc::c_int
                            as libc::c_long != 0
                        {
                            rayStack_capacity = if rayStack_capacity != 0 {
                                rayStack_capacity * 2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                            let mut elemSize: usize = ::core::mem::size_of::<DelayRay>();
                            let mut pData: *mut *mut libc::c_void = &mut rayStack_data
                                as *mut *mut DelayRay as *mut *mut libc::c_void;
                            *pData = MemRealloc(
                                rayStack_data as *mut libc::c_void,
                                (rayStack_capacity as usize).wrapping_mul(elemSize as usize),
                            );
                        }
                        let fresh0 = rayStack_size;
                        rayStack_size = rayStack_size + 1;
                        *rayStack_data.offset(fresh0 as isize) = d;
                        ray.tMax = max;
                    }
                }
            } else if Abs(dist as f64) < 8.0f32 as f64 * 1e-4f64 {
                earlyIndex = nearIndex;
                let mut d_0: DelayRay = {
                    let mut init = DelayRay {
                        nodeRef: (*node).child[(1 as libc::c_int ^ earlyIndex) as usize],
                        tMin: ray.tMin,
                        tMax: ray.tMax,
                        depth: depth,
                    };
                    init
                };
                if (rayStack_capacity == rayStack_size) as libc::c_long
                    != 0
                {
                    rayStack_capacity = if rayStack_capacity != 0 {
                        rayStack_capacity * 2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    let mut elemSize_0: usize = ::core::mem::size_of::<DelayRay>();
                    let mut pData_0: *mut *mut libc::c_void = &mut rayStack_data
                        as *mut *mut DelayRay as *mut *mut libc::c_void;
                    *pData_0 = MemRealloc(
                        rayStack_data as *mut libc::c_void,
                        (rayStack_capacity as usize).wrapping_mul(elemSize_0 as usize),
                    );
                }
                let fresh1 = rayStack_size;
                rayStack_size = rayStack_size + 1;
                *rayStack_data.offset(fresh1 as isize) = d_0;
            }
            depth += 1;
            nodeRef = (*node).child[earlyIndex as usize];
        } else {
            let mut leaf: *const Triangle = ((*this).triangles_data)
                .offset(-nodeRef.index as isize);
            let mut i: uint8 = 0 as libc::c_int as uint8;
            while (i as libc::c_int) < nodeRef.triangleCount as libc::c_int {
                let mut triangle: *const Triangle = leaf
                    .offset(i as libc::c_int as isize);
                let mut t_0: f32 = 0.;
                if Intersect_RayTriangle_Moller1(&mut ray, triangle, &mut t_0) {
                    if !hit || t_0 < *tHit {
                        hit = 1 as libc::c_int != 0;
                        *tHit = t_0;
                    }
                }
                i = i.wrapping_add(1);
            }
            if hit {
                break;
            }
            if rayStack_size == 0 as libc::c_int {
                break;
            }
            rayStack_size -= 1;
            let mut d_1: DelayRay = *rayStack_data.offset(rayStack_size as isize);
            nodeRef = d_1.nodeRef;
            ray.tMin = d_1.tMin;
            ray.tMax = d_1.tMax;
            depth = d_1.depth;
        }
    }
    rayStack_size = 0 as libc::c_int;
    return hit;
}
#[no_mangle]
pub unsafe extern "C" fn BSP_IntersectLineSegment(
    mut this: *mut BSP,
    mut lineSegment: *const LineSegment,
    mut pHit: *mut Vec3,
) -> bool {
    let mut t: f32 = 0.;
    let mut dir: Vec3 = (*lineSegment).p1 - (*lineSegment).p0;
    let mut ray: Ray = {
        let mut init = Ray {
            p: (*lineSegment).p0,
            dir: dir,
            tMin: 0.0f32,
            tMax: 1.0f32,
        };
        init
    };
    if BSP_IntersectRay(this, &mut ray, &mut t) {
        Ray_GetPoint(&mut ray, t, pHit);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub static mut nodeStack_size: int32 = 0;
#[no_mangle]
pub static mut nodeStack_data: *mut Delay = 0 as *const Delay as *mut Delay;
#[no_mangle]
pub static mut nodeStack_capacity: int32 = 0;
#[no_mangle]
pub unsafe extern "C" fn BSP_IntersectSphere(
    mut this: *mut BSP,
    mut sphere: *const Sphere,
    mut pHit: *mut Vec3,
) -> bool {
    let mut nodeRef: BSPNodeRef = (*this).rootNode;
    let mut hit: bool = 0 as libc::c_int != 0;
    let mut depth: int32 = 0 as libc::c_int;
    let mut maxDepth: int32 = 0 as libc::c_int;
    loop {
        maxDepth = Max(depth as f64, maxDepth as f64) as int32;
        if nodeRef.index >= 0 as libc::c_int {
            let mut node: *mut BSPNode = ((*this).nodes_data)
                .offset(nodeRef.index as isize);
            let mut dist: f32 = Vec3::dot((*node).plane.n, (*sphere).p)
                - (*node).plane.d;
            if dist as f64
                > (*sphere).r as f64 + 2.0f32 as f64 * 1e-4f64
            {
                nodeRef = (*node).child[FrontIndex as usize];
            } else if (dist as f64)
                < -((*sphere).r as f64 + 2.0f32 as f64 * 1e-4f64)
            {
                nodeRef = (*node).child[BackIndex as usize];
            } else {
                let mut d: Delay = {
                    let mut init = Delay {
                        nodeRef: (*node).child[BackIndex as usize],
                        depth: depth,
                    };
                    init
                };
                if (nodeStack_capacity == nodeStack_size) as libc::c_long
                    != 0
                {
                    nodeStack_capacity = if nodeStack_capacity != 0 {
                        nodeStack_capacity * 2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    let mut elemSize: usize = ::core::mem::size_of::<Delay>();
                    let mut pData: *mut *mut libc::c_void = &mut nodeStack_data
                        as *mut *mut Delay as *mut *mut libc::c_void;
                    *pData = MemRealloc(
                        nodeStack_data as *mut libc::c_void,
                        (nodeStack_capacity as usize).wrapping_mul(elemSize as usize),
                    );
                }
                let fresh2 = nodeStack_size;
                nodeStack_size = nodeStack_size + 1;
                *nodeStack_data.offset(fresh2 as isize) = d;
                nodeRef = (*node).child[FrontIndex as usize];
            }
            depth += 1;
        } else {
            let mut leaf: *mut Triangle = ((*this).triangles_data)
                .offset(-nodeRef.index as isize);
            let mut i: uint8 = 0 as libc::c_int as uint8;
            while (i as libc::c_int) < nodeRef.triangleCount as libc::c_int {
                let mut triangle: *mut Triangle = leaf.offset(i as libc::c_int as isize);
                let mut pHit2 = Vec3::ZERO;
                if Intersect_SphereTriangle(sphere, triangle, &mut pHit2) {
                    hit = 1 as libc::c_int != 0;
                    *pHit = pHit2;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if hit {
                break;
            }
            if nodeStack_size == 0 as libc::c_int {
                break;
            }
            nodeStack_size -= 1;
            let mut d_0: Delay = *nodeStack_data.offset(nodeStack_size as isize);
            nodeRef = d_0.nodeRef;
            depth = d_0.depth;
        }
    }
    nodeStack_size = 0 as libc::c_int;
    return hit;
}
#[no_mangle]
pub static mut PolygonFlag_None: PolygonFlag = ((0 as libc::c_int) << 0 as libc::c_int)
    as PolygonFlag;
#[no_mangle]
pub static mut PolygonFlag_InvalidFaceSplit: PolygonFlag = ((1 as libc::c_int)
    << 0 as libc::c_int) as PolygonFlag;
#[no_mangle]
pub static mut PolygonFlag_InvalidDecompose: PolygonFlag = ((1 as libc::c_int)
    << 1 as libc::c_int) as PolygonFlag;
#[no_mangle]
pub static mut PolygonFlag_InvalidEdgeSplit: PolygonFlag = ((1 as libc::c_int)
    << 2 as libc::c_int) as PolygonFlag;
unsafe extern "C" fn BSPBuild_ScoreSplitPlane(
    mut nodeData: *mut BSPBuild_NodeData,
    mut plane: Plane,
    mut k: f32,
) -> f32 {
    let mut numInFront: int32 = 0 as libc::c_int;
    let mut numBehind: int32 = 0 as libc::c_int;
    let mut numStraddling: int32 = 0 as libc::c_int;
    let mut polygon: *mut PolygonEx = (*nodeData).polygons_data;
    let mut __iterend: *mut PolygonEx = ((*nodeData).polygons_data)
        .offset((*nodeData).polygons_size as isize);
    while polygon < __iterend {
        let mut classification: PolygonClassification = Plane_ClassifyPolygon(
            &mut plane,
            polygon as *mut Polygon,
        );
        let mut current_block_4: u64;
        match classification as libc::c_int {
            3 | 2 => {
                current_block_4 = 11316911015026613471;
            }
            1 => {
                numInFront += 1;
                current_block_4 = 11812396948646013369;
            }
            4 => {
                numStraddling += 1;
                current_block_4 = 11812396948646013369;
            }
            _ => {
                Fatal(
                    b"BSPBuild_ScoreSplitPlane: Unhandled case: %i\0" as *const u8
                        as *const libc::c_char,
                    classification as libc::c_int,
                );
                current_block_4 = 11316911015026613471;
            }
        }
        match current_block_4 {
            11316911015026613471 => {
                numBehind += 1;
            }
            _ => {}
        }
        polygon = polygon.offset(1);
    }
    let mut score: f32 = Lerp(
        Abs((numInFront - numBehind) as f64) as f32
            as f64,
        numStraddling as f32 as f64,
        k as f64,
    ) as f32;
    return score;
}
unsafe extern "C" fn BSPBuild_ChooseSplitPlane(
    mut bsp: *mut BSPBuild,
    mut nodeData: *mut BSPBuild_NodeData,
    mut splitPlane: *mut Plane,
) -> bool {
    let mut maxDepth: f32 = 1000.0f32;
    let mut biasedDepth: f32 = (*nodeData).depth as f32 - 100.0f32;
    let mut t: f32 = Max(
        (biasedDepth / maxDepth) as f64,
        0.0f32 as f64,
    ) as f32;
    let mut k: f32 = Lerp(
        0.85f32 as f64,
        0.25f32 as f64,
        t as f64,
    ) as f32;
    let mut bestScore: f32 = 3.40282347e+38f32;
    let mut bestPlane: Plane = {
        let mut init = Plane {
            n: Vec3 { x: 0., y: 0., z: 0. },
            d: 0.,
        };
        init
    };
    let mut bestPolygon: *mut PolygonEx = 0 as *mut PolygonEx;
    let mut numToCheck: int32 = 10 as libc::c_int;
    let mut polygonsLen: int32 = (*nodeData).polygons_size;
    if (*nodeData).validPolygonCount > 0 as libc::c_int {
        numToCheck = Min(
            numToCheck as f64,
            (*nodeData).validPolygonCount as f64,
        ) as int32;
        let mut i: int32 = 0 as libc::c_int;
        while i < numToCheck {
            let mut polygonIndex: int32 = (RNG_Get32((*bsp).rng))
                .wrapping_rem(polygonsLen as libc::c_uint) as int32;
            let mut j: int32 = 0 as libc::c_int;
            while j < polygonsLen {
                let mut polygon: *mut PolygonEx = ((*nodeData).polygons_data)
                    .offset(polygonIndex as isize);
                if (*polygon).flags as libc::c_int
                    & PolygonFlag_InvalidFaceSplit as libc::c_int == 0
                {
                    let mut plane: Plane = Plane {
                        n: Vec3 { x: 0., y: 0., z: 0. },
                        d: 0.,
                    };
                    Polygon_ToPlane(polygon as *mut Polygon, &mut plane);
                    let mut score: f32 = BSPBuild_ScoreSplitPlane(
                        nodeData,
                        plane,
                        k,
                    );
                    if score < bestScore {
                        bestScore = score;
                        bestPlane = plane;
                        bestPolygon = polygon;
                    }
                    break;
                } else {
                    polygonIndex = (polygonIndex + 1 as libc::c_int) % polygonsLen;
                    j += 1;
                }
            }
            i += 1;
        }
        if !bestPolygon.is_null() {
            (*bestPolygon)
                .flags = ((*bestPolygon).flags as libc::c_int
                | PolygonFlag_InvalidFaceSplit as libc::c_int) as PolygonFlag;
        }
    } else if polygonsLen > 0 as libc::c_int {
        let mut splitFound: bool = 0 as libc::c_int != 0;
        if !splitFound {
            let mut polygonIndex_0: int32 = (RNG_Get32((*bsp).rng))
                .wrapping_rem(polygonsLen as libc::c_uint) as int32;
            let mut i_0: int32 = 0 as libc::c_int;
            while i_0 < polygonsLen {
                let mut polygon_0: *mut PolygonEx = ((*nodeData).polygons_data)
                    .offset(polygonIndex_0 as isize);
                if !((*polygon_0).flags as libc::c_int
                    & PolygonFlag_InvalidDecompose as libc::c_int != 0)
                {
                    let mut v: *mut Vec3 = (*polygon_0).vertices_data;
                    let mut vLen: int32 = (*polygon_0).vertices_size;
                    let mut j_0: int32 = 2 as libc::c_int;
                    while j_0 < vLen - 1 as libc::c_int {
                        let mut edge: Vec3 = *v.offset(0) - *v.offset(j_0 as isize);
                        let mut mid: Vec3 = Vec3::lerp(
                            *v.offset(0),
                            *v.offset(j_0 as isize),
                            0.5f32,
                        );
                        let mut polygonPlane: Plane = Plane {
                            n: Vec3 { x: 0., y: 0., z: 0. },
                            d: 0.,
                        };
                        Polygon_ToPlane(polygon_0 as *mut Polygon, &mut polygonPlane);
                        let mut plane_0: Plane = Plane {
                            n: Vec3 { x: 0., y: 0., z: 0. },
                            d: 0.,
                        };
                        plane_0.n = Vec3::cross(edge, polygonPlane.n).normalize();
                        plane_0.d = Vec3::dot(plane_0.n, mid);
                        if Plane_ClassifyPolygon(&mut plane_0, polygon_0 as *mut Polygon)
                            as libc::c_int == 4 as libc::c_int
                        {
                            splitFound = 1 as libc::c_int != 0;
                            bestScore = 0.0f32;
                            bestPlane = plane_0;
                            bestPolygon = polygon_0;
                            break;
                        } else {
                            (*polygon_0)
                                .flags = ((*polygon_0).flags as libc::c_int
                                | PolygonFlag_InvalidDecompose as libc::c_int)
                                as PolygonFlag;
                            j_0 += 1;
                        }
                    }
                    if splitFound {
                        break;
                    }
                    polygonIndex_0 = (polygonIndex_0 + 1 as libc::c_int) % polygonsLen;
                }
                i_0 += 1;
            }
            if splitFound {
                (*bestPolygon)
                    .flags = ((*bestPolygon).flags as libc::c_int
                    | PolygonFlag_InvalidDecompose as libc::c_int) as PolygonFlag;
            }
        }
        if !splitFound {
            let mut polygonIndex_1: int32 = (RNG_Get32((*bsp).rng))
                .wrapping_rem(polygonsLen as libc::c_uint) as int32;
            let mut i_1: int32 = 0 as libc::c_int;
            while i_1 < polygonsLen {
                let mut polygon_1: *mut PolygonEx = ((*nodeData).polygons_data)
                    .offset(polygonIndex_1 as isize);
                if !((*polygon_1).flags as libc::c_int
                    & PolygonFlag_InvalidEdgeSplit as libc::c_int != 0)
                {
                    let mut polygonPlane_0: Plane = Plane {
                        n: Vec3 { x: 0., y: 0., z: 0. },
                        d: 0.,
                    };
                    Polygon_ToPlane(polygon_1 as *mut Polygon, &mut polygonPlane_0);
                    let mut v_0: *mut Vec3 = (*polygon_1).vertices_data;
                    let mut vLen_0: int32 = (*polygon_1).vertices_size;
                    let mut vPrev: Vec3 = *v_0
                        .offset((vLen_0 - 1 as libc::c_int) as isize);
                    let mut j_1: int32 = 0 as libc::c_int;
                    while j_1 < vLen_0 {
                        let mut vCur: Vec3 = *v_0.offset(j_1 as isize);
                        let mut edge_0: Vec3 = vCur - vPrev;
                        let mut mid_0: Vec3 = Vec3::lerp(vPrev, vCur, 0.5f32);
                        let mut plane_1: Plane = Plane {
                            n: Vec3 { x: 0., y: 0., z: 0. },
                            d: 0.,
                        };
                        plane_1.n = Vec3::cross(edge_0, polygonPlane_0.n).normalize();
                        plane_1.d = Vec3::dot(plane_1.n, mid_0);
                        let mut score_0: f32 = BSPBuild_ScoreSplitPlane(
                            nodeData,
                            plane_1,
                            0.0f32,
                        );
                        if score_0 < bestScore {
                            splitFound = 1 as libc::c_int != 0;
                            bestPolygon = polygon_1;
                            bestScore = score_0;
                            bestPlane = plane_1;
                        }
                        vPrev = vCur;
                        numToCheck -= 1;
                        if numToCheck == 0 as libc::c_int {
                            break;
                        }
                        j_1 += 1;
                    }
                    if numToCheck == 0 as libc::c_int {
                        break;
                    }
                    polygonIndex_1 = (polygonIndex_1 + 1 as libc::c_int) % polygonsLen;
                }
                i_1 += 1;
            }
            if splitFound {
                (*bestPolygon)
                    .flags = ((*bestPolygon).flags as libc::c_int
                    | PolygonFlag_InvalidEdgeSplit as libc::c_int) as PolygonFlag;
            }
        }
    }
    if bestScore < 3.40282347e+38f32 {
        *splitPlane = bestPlane;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
#[inline]
unsafe extern "C" fn BSPBuild_AppendPolygon(
    mut nodeData: *mut BSPBuild_NodeData,
    mut polygon: *mut PolygonEx,
) {
    (*nodeData).triangleCount += (*polygon).vertices_size - 2 as libc::c_int;
    (*nodeData).validPolygonCount
        += ((*polygon).flags as libc::c_int & PolygonFlag_InvalidFaceSplit as libc::c_int
            == 0) as libc::c_int;
    if ((*nodeData).polygons_capacity == (*nodeData).polygons_size) as libc::c_int
        as libc::c_long != 0
    {
        (*nodeData)
            .polygons_capacity = if (*nodeData).polygons_capacity != 0 {
            (*nodeData).polygons_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<PolygonEx>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*nodeData).polygons_data
            as *mut *mut PolygonEx as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*nodeData).polygons_data as *mut libc::c_void,
            ((*nodeData).polygons_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh3 = (*nodeData).polygons_size;
    (*nodeData).polygons_size = (*nodeData).polygons_size + 1;
    *((*nodeData).polygons_data).offset(fresh3 as isize) = *polygon;
}
unsafe extern "C" fn BSPBuild_CreateNode(
    mut bsp: *mut BSPBuild,
    mut nodeData: *mut BSPBuild_NodeData,
) -> *mut BSPBuild_Node {
    let mut node: *mut BSPBuild_Node = MemAllocZero(
        ::core::mem::size_of::<BSPBuild_Node>() as usize,
    ) as *mut BSPBuild_Node;
    let mut splitPlane: Plane = {
        let mut init = Plane {
            n: Vec3 { x: 0., y: 0., z: 0. },
            d: 0.,
        };
        init
    };
    let mut makeLeaf: bool = 0 as libc::c_int != 0;
    makeLeaf = makeLeaf as libc::c_int != 0
        || (*nodeData).triangleCount <= 12 as libc::c_int;
    makeLeaf = makeLeaf as libc::c_int != 0
        || !BSPBuild_ChooseSplitPlane(bsp, nodeData, &mut splitPlane);
    if makeLeaf {
        if (*nodeData).triangleCount != 0 as libc::c_int {
            (*bsp).leafCount += 1;
        }
        (*bsp).triangleCount += (*nodeData).triangleCount;
        (*node).polygons_capacity = (*nodeData).polygons_capacity;
        (*node).polygons_size = (*nodeData).polygons_size;
        (*node).polygons_data = (*nodeData).polygons_data;
        return node;
    }
    (*bsp).nodeCount += 1;
    let mut polygonsLen: int32 = (*nodeData).polygons_size;
    let mut backNodeData: BSPBuild_NodeData = {
        let mut init = BSPBuild_NodeData {
            polygons_size: 0,
            polygons_capacity: 0,
            polygons_data: 0 as *mut PolygonEx,
            validPolygonCount: 0,
            triangleCount: 0,
            depth: 0,
        };
        init
    };
    if (backNodeData.polygons_capacity < polygonsLen) as libc::c_long != 0
    {
        backNodeData.polygons_capacity = polygonsLen;
        let mut elemSize: usize = ::core::mem::size_of::<PolygonEx>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut backNodeData.polygons_data
            as *mut *mut PolygonEx as *mut *mut libc::c_void;
        *pData = MemRealloc(
            backNodeData.polygons_data as *mut libc::c_void,
            (backNodeData.polygons_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    backNodeData.depth = ((*nodeData).depth as libc::c_int + 1 as libc::c_int) as uint16;
    let mut frontNodeData: BSPBuild_NodeData = {
        let mut init = BSPBuild_NodeData {
            polygons_size: 0,
            polygons_capacity: 0,
            polygons_data: 0 as *mut PolygonEx,
            validPolygonCount: 0,
            triangleCount: 0,
            depth: 0,
        };
        init
    };
    if (frontNodeData.polygons_capacity < polygonsLen) as libc::c_long
        != 0
    {
        frontNodeData.polygons_capacity = polygonsLen;
        let mut elemSize_0: usize = ::core::mem::size_of::<PolygonEx>();
        let mut pData_0: *mut *mut libc::c_void = &mut frontNodeData.polygons_data
            as *mut *mut PolygonEx as *mut *mut libc::c_void;
        *pData_0 = MemRealloc(
            frontNodeData.polygons_data as *mut libc::c_void,
            (frontNodeData.polygons_capacity as usize).wrapping_mul(elemSize_0 as usize),
        );
    }
    frontNodeData
        .depth = ((*nodeData).depth as libc::c_int + 1 as libc::c_int) as uint16;
    let mut polygon: *mut PolygonEx = (*nodeData).polygons_data;
    let mut __iterend: *mut PolygonEx = ((*nodeData).polygons_data)
        .offset((*nodeData).polygons_size as isize);
    while polygon < __iterend {
        let mut classification: PolygonClassification = Plane_ClassifyPolygon(
            &mut splitPlane,
            polygon as *mut Polygon,
        );
        let mut current_block_37: u64;
        match classification as libc::c_int {
            3 => {
                current_block_37 = 18363606670811337990;
            }
            2 => {
                current_block_37 = 1190587995684967772;
            }
            1 => {
                BSPBuild_AppendPolygon(&mut frontNodeData, polygon);
                current_block_37 = 17184638872671510253;
            }
            4 => {
                let mut backPart: PolygonEx = {
                    let mut init = PolygonEx {
                        vertices_size: 0,
                        vertices_capacity: 0,
                        vertices_data: 0 as *mut Vec3,
                        flags: 0,
                    };
                    init
                };
                backPart.flags = (*polygon).flags;
                let mut frontPart: PolygonEx = {
                    let mut init = PolygonEx {
                        vertices_size: 0,
                        vertices_capacity: 0,
                        vertices_data: 0 as *mut Vec3,
                        flags: 0,
                    };
                    init
                };
                frontPart.flags = (*polygon).flags;
                Polygon_SplitSafe(
                    polygon as *mut Polygon,
                    splitPlane,
                    &mut backPart as *mut PolygonEx as *mut Polygon,
                    &mut frontPart as *mut PolygonEx as *mut Polygon,
                );
                BSPBuild_AppendPolygon(&mut backNodeData, &mut backPart);
                BSPBuild_AppendPolygon(&mut frontNodeData, &mut frontPart);
                MemFree((*polygon).vertices_data as *const libc::c_void);
                current_block_37 = 17184638872671510253;
            }
            _ => {
                Fatal(
                    b"BSPBuild_CreateNode: Unhandled case: %i\0" as *const u8
                        as *const libc::c_char,
                    classification as libc::c_int,
                );
                current_block_37 = 18363606670811337990;
            }
        }
        match current_block_37 {
            18363606670811337990 => {
                (*polygon)
                    .flags = ((*polygon).flags as libc::c_int
                    | PolygonFlag_InvalidFaceSplit as libc::c_int) as PolygonFlag;
                current_block_37 = 1190587995684967772;
            }
            _ => {}
        }
        match current_block_37 {
            1190587995684967772 => {
                BSPBuild_AppendPolygon(&mut backNodeData, polygon);
            }
            _ => {}
        }
        polygon = polygon.offset(1);
    }
    MemFree((*nodeData).polygons_data as *const libc::c_void);
    (*node).plane = splitPlane;
    (*node).child[BackIndex as usize] = BSPBuild_CreateNode(bsp, &mut backNodeData);
    (*node).child[FrontIndex as usize] = BSPBuild_CreateNode(bsp, &mut frontNodeData);
    return node;
}
unsafe extern "C" fn BSPBuild_OptimizeTree(
    mut this: *mut BSP,
    mut buildNode: *mut BSPBuild_Node,
) -> BSPNodeRef {
    if !((*buildNode).child[BackIndex as usize]).is_null()
        || !((*buildNode).child[FrontIndex as usize]).is_null()
    {
        let mut dummy: BSPNode = {
            let mut init = BSPNode {
                plane: Plane {
                    n: Vec3 { x: 0., y: 0., z: 0. },
                    d: 0.,
                },
                child: [BSPNodeRef {
                    index: 0,
                    triangleCount: 0,
                }; 2],
            };
            init
        };
        let mut nodeIndex: int32 = (*this).nodes_size;
        if ((*this).nodes_capacity == (*this).nodes_size) as libc::c_int
            as libc::c_long != 0
        {
            (*this)
                .nodes_capacity = if (*this).nodes_capacity != 0 {
                (*this).nodes_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize: usize = ::core::mem::size_of::<BSPNode>();
            let mut pData: *mut *mut libc::c_void = &mut (*this).nodes_data
                as *mut *mut BSPNode as *mut *mut libc::c_void;
            *pData = MemRealloc(
                (*this).nodes_data as *mut libc::c_void,
                ((*this).nodes_capacity as usize).wrapping_mul(elemSize as usize),
            );
        }
        let fresh4 = (*this).nodes_size;
        (*this).nodes_size = (*this).nodes_size + 1;
        *((*this).nodes_data).offset(fresh4 as isize) = dummy;
        let mut node: *mut BSPNode = ((*this).nodes_data)
            .offset((*this).nodes_size as isize)
            .offset(-(1));
        (*node).plane = (*buildNode).plane;
        (*node)
            .child[BackIndex
            as usize] = BSPBuild_OptimizeTree(
            this,
            (*buildNode).child[BackIndex as usize],
        );
        (*node)
            .child[FrontIndex
            as usize] = BSPBuild_OptimizeTree(
            this,
            (*buildNode).child[FrontIndex as usize],
        );
        let mut result: BSPNodeRef = {
            let mut init = BSPNodeRef {
                index: nodeIndex,
                triangleCount: 0 as libc::c_int as uint8,
            };
            init
        };
        return result;
    } else {
        if (*buildNode).polygons_size == 0 as libc::c_int {
            return (*this).emptyLeaf;
        }
        let mut leafIndex: int32 = (*this).triangles_size;
        let mut polygon: *mut PolygonEx = (*buildNode).polygons_data;
        let mut __iterend: *mut PolygonEx = ((*buildNode).polygons_data)
            .offset((*buildNode).polygons_size as isize);
        while polygon < __iterend {
            Polygon_ConvexToTriangles(
                polygon as *mut Polygon,
                &mut (*this).triangles_capacity,
                &mut (*this).triangles_size,
                &mut (*this).triangles_data,
            );
            polygon = polygon.offset(1);
        }
        let mut leafLen: uint8 = ((*this).triangles_size - leafIndex) as uint8;
        let mut result_0: BSPNodeRef = {
            let mut init = BSPNodeRef {
                index: -leafIndex,
                triangleCount: leafLen,
            };
            init
        };
        return result_0;
    };
}
unsafe extern "C" fn BSPBuild_FreeNode(mut node: *mut BSPBuild_Node) {
    if !((*node).child[BackIndex as usize]).is_null()
        || !((*node).child[FrontIndex as usize]).is_null()
    {
        BSPBuild_FreeNode((*node).child[BackIndex as usize]);
        BSPBuild_FreeNode((*node).child[FrontIndex as usize]);
    } else {
        let mut polygon: *mut PolygonEx = (*node).polygons_data;
        let mut __iterend: *mut PolygonEx = ((*node).polygons_data)
            .offset((*node).polygons_size as isize);
        while polygon < __iterend {
            MemFree((*polygon).vertices_data as *const libc::c_void);
            polygon = polygon.offset(1);
        }
        MemFree((*node).polygons_data as *const libc::c_void);
    }
    MemFree(node as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BSP_Create(mut mesh: *mut Mesh) -> *mut BSP {
    let mut this: *mut BSP = MemAllocZero(
        ::core::mem::size_of::<BSP>(),
    ) as *mut BSP;
    let mut indexLen: int32 = Mesh_GetIndexCount(mesh);
    let mut indexData: *mut int32 = Mesh_GetIndexData(mesh);
    let mut vertexData: *mut Vertex = Mesh_GetVertexData(mesh);
    let mut nodeData: BSPBuild_NodeData = {
        let mut init = BSPBuild_NodeData {
            polygons_size: 0,
            polygons_capacity: 0,
            polygons_data: 0 as *mut PolygonEx,
            validPolygonCount: 0,
            triangleCount: 0,
            depth: 0,
        };
        init
    };
    nodeData.triangleCount = indexLen / 3 as libc::c_int;
    nodeData.validPolygonCount = indexLen / 3 as libc::c_int;
    if (nodeData.polygons_capacity < nodeData.triangleCount) as libc::c_int
        as libc::c_long != 0
    {
        nodeData.polygons_capacity = nodeData.triangleCount;
        let mut elemSize: usize = ::core::mem::size_of::<PolygonEx>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut nodeData.polygons_data
            as *mut *mut PolygonEx as *mut *mut libc::c_void;
        *pData = MemRealloc(
            nodeData.polygons_data as *mut libc::c_void,
            (nodeData.polygons_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let mut i: int32 = 0 as libc::c_int;
    while i < indexLen {
        let mut i0: int32 = *indexData.offset((i + 0 as libc::c_int) as isize);
        let mut i1: int32 = *indexData.offset((i + 1 as libc::c_int) as isize);
        let mut i2: int32 = *indexData.offset((i + 2 as libc::c_int) as isize);
        let mut v0: Vec3 = (*vertexData.offset(i0 as isize)).p;
        let mut v1: Vec3 = (*vertexData.offset(i1 as isize)).p;
        let mut v2: Vec3 = (*vertexData.offset(i2 as isize)).p;
        let mut polygon: PolygonEx = {
            let mut init = PolygonEx {
                vertices_size: 0,
                vertices_capacity: 0,
                vertices_data: 0 as *mut Vec3,
                flags: 0,
            };
            init
        };
        if (polygon.vertices_capacity < 3 as libc::c_int) as libc::c_long
            != 0
        {
            polygon.vertices_capacity = 3 as libc::c_int;
            let mut elemSize_0: usize = ::core::mem::size_of::<Vec3>();
            let mut pData_0: *mut *mut libc::c_void = &mut polygon.vertices_data
                as *mut *mut Vec3 as *mut *mut libc::c_void;
            *pData_0 = MemRealloc(
                polygon.vertices_data as *mut libc::c_void,
                (polygon.vertices_capacity as usize).wrapping_mul(elemSize_0 as usize),
            );
        }
        if (polygon.vertices_capacity == polygon.vertices_size) as libc::c_int
            as libc::c_long != 0
        {
            polygon
                .vertices_capacity = if polygon.vertices_capacity != 0 {
                polygon.vertices_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize_1: usize = ::core::mem::size_of::<Vec3>();
            let mut pData_1: *mut *mut libc::c_void = &mut polygon.vertices_data
                as *mut *mut Vec3 as *mut *mut libc::c_void;
            *pData_1 = MemRealloc(
                polygon.vertices_data as *mut libc::c_void,
                (polygon.vertices_capacity as usize).wrapping_mul(elemSize_1 as usize),
            );
        }
        let fresh5 = polygon.vertices_size;
        polygon.vertices_size = polygon.vertices_size + 1;
        *(polygon.vertices_data).offset(fresh5 as isize) = v0;
        if (polygon.vertices_capacity == polygon.vertices_size) as libc::c_int
            as libc::c_long != 0
        {
            polygon
                .vertices_capacity = if polygon.vertices_capacity != 0 {
                polygon.vertices_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize_2: usize = ::core::mem::size_of::<Vec3>();
            let mut pData_2: *mut *mut libc::c_void = &mut polygon.vertices_data
                as *mut *mut Vec3 as *mut *mut libc::c_void;
            *pData_2 = MemRealloc(
                polygon.vertices_data as *mut libc::c_void,
                (polygon.vertices_capacity as usize).wrapping_mul(elemSize_2 as usize),
            );
        }
        let fresh6 = polygon.vertices_size;
        polygon.vertices_size = polygon.vertices_size + 1;
        *(polygon.vertices_data).offset(fresh6 as isize) = v1;
        if (polygon.vertices_capacity == polygon.vertices_size) as libc::c_int
            as libc::c_long != 0
        {
            polygon
                .vertices_capacity = if polygon.vertices_capacity != 0 {
                polygon.vertices_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize_3: usize = ::core::mem::size_of::<Vec3>();
            let mut pData_3: *mut *mut libc::c_void = &mut polygon.vertices_data
                as *mut *mut Vec3 as *mut *mut libc::c_void;
            *pData_3 = MemRealloc(
                polygon.vertices_data as *mut libc::c_void,
                (polygon.vertices_capacity as usize).wrapping_mul(elemSize_3 as usize),
            );
        }
        let fresh7 = polygon.vertices_size;
        polygon.vertices_size = polygon.vertices_size + 1;
        *(polygon.vertices_data).offset(fresh7 as isize) = v2;
        if (nodeData.polygons_capacity == nodeData.polygons_size) as libc::c_int
            as libc::c_long != 0
        {
            nodeData
                .polygons_capacity = if nodeData.polygons_capacity != 0 {
                nodeData.polygons_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize_4: usize = ::core::mem::size_of::<PolygonEx>();
            let mut pData_4: *mut *mut libc::c_void = &mut nodeData.polygons_data
                as *mut *mut PolygonEx as *mut *mut libc::c_void;
            *pData_4 = MemRealloc(
                nodeData.polygons_data as *mut libc::c_void,
                (nodeData.polygons_capacity as usize).wrapping_mul(elemSize_4 as usize),
            );
        }
        let fresh8 = nodeData.polygons_size;
        nodeData.polygons_size = nodeData.polygons_size + 1;
        *(nodeData.polygons_data).offset(fresh8 as isize) = polygon;
        i += 3 as libc::c_int;
    }
    let mut bspBuild: BSPBuild = {
        let mut init = BSPBuild {
            rootNode: 0 as *mut BSPBuild_Node,
            rng: 0 as *mut RNG,
            nodeCount: 0,
            leafCount: 0,
            triangleCount: 0,
        };
        init
    };
    bspBuild.rng = RNG_Create(1235 as libc::c_int as uint64);
    bspBuild.rootNode = BSPBuild_CreateNode(&mut bspBuild, &mut nodeData);
    let mut nullLeaf: Triangle = {
        let mut init = Triangle {
            vertices: [Vec3 { x: 0., y: 0., z: 0. }; 3],
        };
        init
    };
    if ((*this).triangles_capacity < bspBuild.triangleCount + 2 as libc::c_int)
        as libc::c_long != 0
    {
        (*this).triangles_capacity = bspBuild.triangleCount + 2 as libc::c_int;
        let mut elemSize_5: usize = ::core::mem::size_of::<Triangle>() as usize;
        let mut pData_5: *mut *mut libc::c_void = &mut (*this).triangles_data
            as *mut *mut Triangle as *mut *mut libc::c_void;
        *pData_5 = MemRealloc(
            (*this).triangles_data as *mut libc::c_void,
            ((*this).triangles_capacity as usize).wrapping_mul(elemSize_5 as usize),
        );
    }
    if ((*this).triangles_capacity == (*this).triangles_size) as libc::c_int
        as libc::c_long != 0
    {
        (*this)
            .triangles_capacity = if (*this).triangles_capacity != 0 {
            (*this).triangles_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize_6: usize = ::core::mem::size_of::<Triangle>() as usize;
        let mut pData_6: *mut *mut libc::c_void = &mut (*this).triangles_data
            as *mut *mut Triangle as *mut *mut libc::c_void;
        *pData_6 = MemRealloc(
            (*this).triangles_data as *mut libc::c_void,
            ((*this).triangles_capacity as usize).wrapping_mul(elemSize_6 as usize),
        );
    }
    let fresh9 = (*this).triangles_size;
    (*this).triangles_size = (*this).triangles_size + 1;
    *((*this).triangles_data).offset(fresh9 as isize) = nullLeaf;
    if ((*this).triangles_capacity == (*this).triangles_size) as libc::c_int
        as libc::c_long != 0
    {
        (*this)
            .triangles_capacity = if (*this).triangles_capacity != 0 {
            (*this).triangles_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize_7: usize = ::core::mem::size_of::<Triangle>() as usize;
        let mut pData_7: *mut *mut libc::c_void = &mut (*this).triangles_data
            as *mut *mut Triangle as *mut *mut libc::c_void;
        *pData_7 = MemRealloc(
            (*this).triangles_data as *mut libc::c_void,
            ((*this).triangles_capacity as usize).wrapping_mul(elemSize_7 as usize),
        );
    }
    let fresh10 = (*this).triangles_size;
    (*this).triangles_size = (*this).triangles_size + 1;
    *((*this).triangles_data).offset(fresh10 as isize) = nullLeaf;
    (*this).emptyLeaf.index = -EmptyLeafIndex;
    (*this).emptyLeaf.triangleCount = 0 as libc::c_int as uint8;
    let mut nullNode: BSPNode = {
        let mut init = BSPNode {
            plane: Plane {
                n: Vec3 { x: 0., y: 0., z: 0. },
                d: 0.,
            },
            child: [BSPNodeRef {
                index: 0,
                triangleCount: 0,
            }; 2],
        };
        init
    };
    if ((*this).nodes_capacity < bspBuild.nodeCount + 1 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        (*this).nodes_capacity = bspBuild.nodeCount + 1 as libc::c_int;
        let mut elemSize_8: usize = ::core::mem::size_of::<BSPNode>() as usize;
        let mut pData_8: *mut *mut libc::c_void = &mut (*this).nodes_data
            as *mut *mut BSPNode as *mut *mut libc::c_void;
        *pData_8 = MemRealloc(
            (*this).nodes_data as *mut libc::c_void,
            ((*this).nodes_capacity as usize).wrapping_mul(elemSize_8 as usize),
        );
    }
    if ((*this).nodes_capacity == (*this).nodes_size) as libc::c_long
        != 0
    {
        (*this)
            .nodes_capacity = if (*this).nodes_capacity != 0 {
            (*this).nodes_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize_9: usize = ::core::mem::size_of::<BSPNode>() as usize;
        let mut pData_9: *mut *mut libc::c_void = &mut (*this).nodes_data
            as *mut *mut BSPNode as *mut *mut libc::c_void;
        *pData_9 = MemRealloc(
            (*this).nodes_data as *mut libc::c_void,
            ((*this).nodes_capacity as usize).wrapping_mul(elemSize_9 as usize),
        );
    }
    let fresh11 = (*this).nodes_size;
    (*this).nodes_size = (*this).nodes_size + 1;
    *((*this).nodes_data).offset(fresh11 as isize) = nullNode;
    (*this).rootNode = BSPBuild_OptimizeTree(this, bspBuild.rootNode);
    BSPBuild_FreeNode(bspBuild.rootNode);
    RNG_Free(bspBuild.rng);
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn BSP_Free(mut this: *mut BSP) {
    if this.is_null() {
        return;
    }
    MemFree((*this).nodes_data as *const libc::c_void);
    MemFree((*this).triangles_data as *const libc::c_void);
    MemFree(this as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_GetNode(
    mut this: *mut BSP,
    mut nodeRef: BSPNodeRef,
    mut relationship: BSPNodeRel,
) -> BSPNodeRef {
    if this.is_null() {
        Fatal(b"BSP_GetNode: bsp is null\0" as *const u8 as *const libc::c_char);
    }
    if nodeRef.index == 0 {
        return (*this).rootNode;
    }
    let mut node: *mut BSPNode = 0 as *mut BSPNode;
    if nodeRef.index > 0 as libc::c_int {
        node = ((*this).nodes_data).offset(nodeRef.index as isize);
    }
    let mut newNode: BSPNodeRef = {
        let mut init = BSPNodeRef {
            index: 0,
            triangleCount: 0,
        };
        init
    };
    let mut current_block_15: u64;
    match relationship as libc::c_int {
        0 => {
            current_block_15 = 1626635900302357725;
        }
        1 => {
            if !node.is_null() {
                newNode = (*node).child[BackIndex as usize];
            }
            current_block_15 = 4495394744059808450;
        }
        2 => {
            if !node.is_null() {
                newNode = (*node).child[FrontIndex as usize];
            }
            current_block_15 = 4495394744059808450;
        }
        _ => {
            Fatal(
                b"BSPDebug_GetNode: Unhandled case: %i\0" as *const u8
                    as *const libc::c_char,
                relationship as libc::c_int,
            );
            current_block_15 = 1626635900302357725;
        }
    }
    match current_block_15 {
        1626635900302357725 => {
            if nodeRef.index != 0 {
                let mut i: int32 = 0 as libc::c_int;
                while i < (*this).nodes_size {
                    let mut nodeToCheck: *mut BSPNode = ((*this).nodes_data)
                        .offset(i as isize);
                    if (*nodeToCheck).child[BackIndex as usize].index == nodeRef.index {
                        newNode.index = i;
                        break;
                    } else if (*nodeToCheck).child[FrontIndex as usize].index
                        == nodeRef.index
                    {
                        newNode.index = i;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        _ => {}
    }
    return if newNode.index != 0 { newNode } else { nodeRef };
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_DrawNode(
    mut this: *mut BSP,
    mut nodeRef: BSPNodeRef,
) {
    if nodeRef.index > 0 as libc::c_int {
        let mut node: *mut BSPNode = ((*this).nodes_data)
            .offset(nodeRef.index as isize);
        BSPDebug_DrawNode(this, (*node).child[BackIndex as usize]);
        BSPDebug_DrawNode(this, (*node).child[FrontIndex as usize]);
    } else {
        let mut leaf: *mut Triangle = ((*this).triangles_data)
            .offset(-nodeRef.index as isize);
        let mut i: uint8 = 0 as libc::c_int as uint8;
        while (i as libc::c_int) < nodeRef.triangleCount as libc::c_int {
            let mut triangle: *mut Triangle = leaf.offset(i as libc::c_int as isize);
            Draw_Poly3(((*triangle).vertices).as_mut_ptr(), 3 as libc::c_int);
            i = i.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_DrawNodeSplit(
    mut this: *mut BSP,
    mut nodeRef: BSPNodeRef,
) {
    RenderState_PushBlendMode(1 as libc::c_int);
    RenderState_PushCullFace(1 as libc::c_int);
    RenderState_PushDepthTest(1 as libc::c_int != 0);
    RenderState_PushWireframe(1 as libc::c_int != 0);
    if nodeRef.index > 0 as libc::c_int {
        let mut node: *mut BSPNode = ((*this).nodes_data)
            .offset(nodeRef.index as isize);
        Draw_Color(0.5f32, 0.3f32, 0.3f32, 0.4f32);
        BSPDebug_DrawNode(this, (*node).child[BackIndex as usize]);
        Draw_Color(0.3f32, 0.5f32, 0.3f32, 0.4f32);
        BSPDebug_DrawNode(this, (*node).child[FrontIndex as usize]);
        let mut closestPoint = Vec3::ZERO;
        let mut origin: Vec3 = {
            let mut init = Vec3 { x: 0., y: 0., z: 0. };
            init
        };
        let mut t: f32 = Vec3::dot((*node).plane.n, origin) - (*node).plane.d;
        closestPoint = origin - ((*node).plane.n * t);
        RenderState_PushWireframe(0 as libc::c_int != 0);
        Draw_Color(0.3f32, 0.5f32, 0.3f32, 0.4f32);
        Draw_Plane(
            &mut closestPoint,
            &mut (*node).plane.n,
            2.0f32,
        );
        Draw_Color(0.5f32, 0.3f32, 0.3f32, 0.4f32);
        let mut neg: Vec3 = (*node).plane.n * -1.0f32;
        Draw_Plane(&mut closestPoint, &mut neg, 2.0f32);
        RenderState_PopWireframe();
    } else {
        Draw_Color(0.5f32, 0.5f32, 0.3f32, 0.4f32);
        BSPDebug_DrawNode(this, nodeRef);
    }
    RenderState_PopWireframe();
    RenderState_PopDepthTest();
    RenderState_PopCullFace();
    RenderState_PopBlendMode();
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_DrawLineSegment(
    mut bsp: *mut BSP,
    mut lineSegment: *mut LineSegment,
) {
    let mut pHit = Vec3::ZERO;
    if BSP_IntersectLineSegment(bsp, lineSegment, &mut pHit) {
        Draw_Color(0.0f32, 1.0f32, 0.0f32, 0.1f32);
        Draw_Line3(&mut (*lineSegment).p0, &mut pHit);
        Draw_Color(1.0f32, 0.0f32, 0.0f32, 1.0f32);
        Draw_Line3(&mut pHit, &mut (*lineSegment).p1);
        Draw_PointSize(5.0f32);
        Draw_Point3(pHit.x, pHit.y, pHit.z);
    } else {
        Draw_Color(0.0f32, 1.0f32, 0.0f32, 1.0f32);
        Draw_Line3(&mut (*lineSegment).p0, &mut (*lineSegment).p1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_DrawSphere(
    mut this: *mut BSP,
    mut sphere: *mut Sphere,
) {
    let mut pHit = Vec3::ZERO;
    if BSP_IntersectSphere(this, sphere, &mut pHit) {
        RenderState_PushWireframe(0 as libc::c_int != 0);
        Draw_Color(1.0f32, 0.0f32, 0.0f32, 0.3f32);
        Draw_Sphere(&mut (*sphere).p, (*sphere).r);
        RenderState_PopWireframe();
        Draw_Color(1.0f32, 0.0f32, 0.0f32, 1.0f32);
        Draw_Sphere(&mut (*sphere).p, (*sphere).r);
        RenderState_PushDepthTest(0 as libc::c_int != 0);
        Draw_PointSize(8.0f32);
        Draw_Point3(pHit.x, pHit.y, pHit.z);
        RenderState_PopDepthTest();
    } else {
        RenderState_PushWireframe(0 as libc::c_int != 0);
        Draw_Color(0.0f32, 1.0f32, 0.0f32, 0.3f32);
        Draw_Sphere(&mut (*sphere).p, (*sphere).r);
        RenderState_PopWireframe();
        Draw_Color(0.0f32, 1.0f32, 0.0f32, 1.0f32);
        Draw_Sphere(&mut (*sphere).p, (*sphere).r);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_PrintRayProfilingData(
    mut this: *mut BSP,
    mut totalTime: f64,
) {
    Warn(
        b"BSP_PrintRayProfilingData: BSP profiling is not enabled. Set ENABLE_BSP_PROFILING to enable this function.\0"
            as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_PrintSphereProfilingData(
    mut this: *mut BSP,
    mut totalTime: f64,
) {
    Warn(
        b"BSP_PrintSphereProfilingData: BSP profiling is not enabled. Set ENABLE_BSP_PROFILING to enable this function.\0"
            as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_GetIntersectSphereTriangles(
    mut this: *mut BSP,
    mut sphere: *mut Sphere,
    mut sphereProf: *mut IntersectSphereProfiling,
) -> bool {
    let mut nodeRef: BSPNodeRef = (*this).rootNode;
    let mut hit: bool = 0 as libc::c_int != 0;
    let mut depth: int32 = 0 as libc::c_int;
    let mut maxDepth: int32 = 0 as libc::c_int;
    loop {
        maxDepth = Max(depth as f64, maxDepth as f64) as int32;
        if nodeRef.index >= 0 as libc::c_int {
            let mut node: *mut BSPNode = ((*this).nodes_data)
                .offset(nodeRef.index as isize);
            (*sphereProf).nodes += 1;
            let mut dist: f32 = Vec3::dot((*node).plane.n, (*sphere).p)
                - (*node).plane.d;
            if dist as f64
                > (*sphere).r as f64 + 2.0f32 as f64 * 1e-4f64
            {
                nodeRef = (*node).child[FrontIndex as usize];
            } else if (dist as f64)
                < -((*sphere).r as f64 + 2.0f32 as f64 * 1e-4f64)
            {
                nodeRef = (*node).child[BackIndex as usize];
            } else {
                let mut d: Delay = {
                    let mut init = Delay {
                        nodeRef: (*node).child[BackIndex as usize],
                        depth: depth,
                    };
                    init
                };
                if (nodeStack_capacity == nodeStack_size) as libc::c_long
                    != 0
                {
                    nodeStack_capacity = if nodeStack_capacity != 0 {
                        nodeStack_capacity * 2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    let mut elemSize: usize = ::core::mem::size_of::<Delay>();
                    let mut pData: *mut *mut libc::c_void = &mut nodeStack_data
                        as *mut *mut Delay as *mut *mut libc::c_void;
                    *pData = MemRealloc(
                        nodeStack_data as *mut libc::c_void,
                        (nodeStack_capacity as usize).wrapping_mul(elemSize as usize),
                    );
                }
                let fresh12 = nodeStack_size;
                nodeStack_size = nodeStack_size + 1;
                *nodeStack_data.offset(fresh12 as isize) = d;
                nodeRef = (*node).child[FrontIndex as usize];
            }
            depth += 1;
        } else {
            let mut leaf: *mut Triangle = ((*this).triangles_data)
                .offset(-nodeRef.index as isize);
            (*sphereProf).leaves += 1;
            let mut i: uint8 = 0 as libc::c_int as uint8;
            while (i as libc::c_int) < nodeRef.triangleCount as libc::c_int {
                let mut triangle: *mut Triangle = leaf.offset(i as libc::c_int as isize);
                (*sphereProf).triangles += 1;
                let mut pHit2 = Vec3::ZERO;
                if Intersect_SphereTriangle(sphere, triangle, &mut pHit2) {
                    let mut t: TriangleTest = {
                        let mut init = TriangleTest {
                            triangle: triangle,
                            hit: 1 as libc::c_int != 0,
                        };
                        init
                    };
                    if ((*sphereProf).triangleTests_capacity
                        == (*sphereProf).triangleTests_size) as libc::c_int
                        as libc::c_long != 0
                    {
                        (*sphereProf)
                            .triangleTests_capacity = if (*sphereProf)
                            .triangleTests_capacity != 0
                        {
                            (*sphereProf).triangleTests_capacity * 2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        };
                        let mut elemSize_0: usize = ::core::mem::size_of::<
                            TriangleTest,
                        >();
                        let mut pData_0: *mut *mut libc::c_void = &mut (*sphereProf)
                            .triangleTests_data as *mut *mut TriangleTest
                            as *mut *mut libc::c_void;
                        *pData_0 = MemRealloc(
                            (*sphereProf).triangleTests_data as *mut libc::c_void,
                            ((*sphereProf).triangleTests_capacity as usize)
                                .wrapping_mul(elemSize_0),
                        );
                    }
                    let fresh13 = (*sphereProf).triangleTests_size;
                    (*sphereProf)
                        .triangleTests_size = (*sphereProf).triangleTests_size + 1;
                    *((*sphereProf).triangleTests_data).offset(fresh13 as isize) = t;
                    hit = 1 as libc::c_int != 0;
                    break;
                } else {
                    let mut t_0: TriangleTest = {
                        let mut init = TriangleTest {
                            triangle: triangle,
                            hit: 0 as libc::c_int != 0,
                        };
                        init
                    };
                    if ((*sphereProf).triangleTests_capacity
                        == (*sphereProf).triangleTests_size) as libc::c_int
                        as libc::c_long != 0
                    {
                        (*sphereProf)
                            .triangleTests_capacity = if (*sphereProf)
                            .triangleTests_capacity != 0
                        {
                            (*sphereProf).triangleTests_capacity * 2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        };
                        let mut elemSize_1: usize = ::core::mem::size_of::<
                            TriangleTest,
                        >();
                        let mut pData_1: *mut *mut libc::c_void = &mut (*sphereProf)
                            .triangleTests_data as *mut *mut TriangleTest
                            as *mut *mut libc::c_void;
                        *pData_1 = MemRealloc(
                            (*sphereProf).triangleTests_data as *mut libc::c_void,
                            ((*sphereProf).triangleTests_capacity as usize)
                                .wrapping_mul(elemSize_1),
                        );
                    }
                    let fresh14 = (*sphereProf).triangleTests_size;
                    (*sphereProf)
                        .triangleTests_size = (*sphereProf).triangleTests_size + 1;
                    *((*sphereProf).triangleTests_data).offset(fresh14 as isize) = t_0;
                    i = i.wrapping_add(1);
                }
            }
            if hit {
                break;
            }
            if nodeStack_size == 0 as libc::c_int {
                break;
            }
            nodeStack_size -= 1;
            let mut d_0: Delay = *nodeStack_data.offset(nodeStack_size as isize);
            nodeRef = d_0.nodeRef;
            depth = d_0.depth;
        }
    }
    nodeStack_size = 0 as libc::c_int;
    return hit;
}
#[no_mangle]
pub unsafe extern "C" fn BSPDebug_GetLeaf(
    mut this: *mut BSP,
    mut leafIndex: int32,
) -> BSPNodeRef {
    let mut index: int32 = -(1 as libc::c_int);
    let mut node: *mut BSPNode = (*this).nodes_data;
    let mut __iterend: *mut BSPNode = ((*this).nodes_data)
        .offset((*this).nodes_size as isize);
    while node < __iterend {
        if (*node).child[0].index < 0 as libc::c_int {
            let fresh15 = index;
            index = index + 1;
            if fresh15 == leafIndex {
                return (*node).child[0];
            }
        }
        if (*node).child[1].index < 0 as libc::c_int {
            let fresh16 = index;
            index = index + 1;
            if fresh16 == leafIndex {
                return (*node).child[1];
            }
        }
        node = node.offset(1);
    }
    let mut result: BSPNodeRef = {
        let mut init = BSPNodeRef {
            index: RootNodeIndex,
            triangleCount: 0 as libc::c_int as uint8,
        };
        init
    };
    return result;
}
