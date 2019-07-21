use crate::common::rand;
use crate::geometry::{dot, fmax, fmin, Ray, Vec3};
use std::f32;
use std::mem::swap;
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub mat: Option<Rc<dyn Material>>,
    pub u: f32,
    pub v: f32,
}
impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::new(0f32, 0f32, 0f32),
            normal: Vec3::new(0f32, 0f32, 0f32),
            t: 0f32,
            mat: None,
            u: 0f32,
            v: 0f32,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool;
}

pub struct HitList {
    pub list: Vec<Rc<dyn Hitable>>,
}
impl HitList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}
impl Hitable for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            mat: rec.mat.clone(),
            ..*rec
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if h.hit(ray, t_min, closest_so_far, &mut temp_rec) == true {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.mat = temp_rec.mat.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool {
        if self.list.len() < 1 {
            return false;
        }
        let mut temp_box = AABB::new();
        let first_true = self.list[0].bounding_box(t0, t1, &mut temp_box);
        if first_true == false {
            return false;
        } else {
            *b = temp_box;
        }
        for i in 1..self.list.len() {
            if self.list[i].bounding_box(t0, t1, &mut temp_box) == true {
                *b = surrounding_box(b, &temp_box);
            } else {
                return false;
            }
        }
        true
    }
}

fn get_sphere_uv(p: &Vec3, u: &mut f32, v: &mut f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    *u = 1.0 - (phi + f32::consts::PI) / (2.0 / f32::consts::PI);
    *u = (theta + f32::consts::PI / 2.0) / f32::consts::PI;
}

#[derive(Clone)]
pub struct SphereObject {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl Hitable for SphereObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.o - self.center;
        let a = dot(&ray.d, &ray.d);
        let b = dot(&oc, &ray.d);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0f32 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat.clone();
                get_sphere_uv(&rec.p, &mut rec.u, &mut rec.v);
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat.clone();
                get_sphere_uv(&rec.p, &mut rec.u, &mut rec.v);
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, _t0: f32, _t1: f32, b: &mut AABB) -> bool {
        b.lower = self.center - Vec3::new(self.radius, self.radius, self.radius);
        b.upper = self.center + Vec3::new(self.radius, self.radius, self.radius);
        true
    }
}

pub struct MovingSphereObject {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl MovingSphereObject {
    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}
impl Hitable for MovingSphereObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.o - self.center(ray.time);
        let a = dot(&ray.d, &ray.d);
        let b = dot(&oc, &ray.d);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0f32 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center(ray.time)) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center(ray.time)) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool {
        let mut box0 = AABB::new();
        box0.lower = self.center(t0) - Vec3::new(self.radius, self.radius, self.radius);
        box0.upper = self.center(t0) + Vec3::new(self.radius, self.radius, self.radius);
        let mut box1 = AABB::new();
        box1.lower = self.center(t1) - Vec3::new(self.radius, self.radius, self.radius);
        box1.upper = self.center(t1) + Vec3::new(self.radius, self.radius, self.radius);
        *b = surrounding_box(&box0, &box1);
        true
    }
}

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl Hitable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.o.z) / ray.d.z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.o.x + t * ray.d.x;
        let y = ray.o.y + t * ray.d.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.mat = self.mat.clone();
        rec.p = ray.point_at_parameter(t);
        rec.normal = Vec3::new(0.0, 0.0, 1.0);
        true
    }
    fn bounding_box(&self, _t0: f32, _t1: f32, b: &mut AABB) -> bool {
        *b = AABB::new();
        b.lower = Vec3::new(self.x0, self.y0, self.k - 0.0001);
        b.upper = Vec3::new(self.x1, self.y1, self.k + 0.0001);
        true
    }
}
pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl Hitable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.o.y) / ray.d.y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.o.x + t * ray.d.x;
        let z = ray.o.z + t * ray.d.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.mat = self.mat.clone();
        rec.p = ray.point_at_parameter(t);
        rec.normal = Vec3::new(0.0, 1.0, 0.0);
        true
    }
    fn bounding_box(&self, _t0: f32, _t1: f32, b: &mut AABB) -> bool {
        *b = AABB::new();
        b.lower = Vec3::new(self.x0, self.k - 0.0001, self.z0);
        b.upper = Vec3::new(self.x1, self.k + 0.0001, self.z1);
        true
    }
}
pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl Hitable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.o.x) / ray.d.x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.o.y + t * ray.d.y;
        let z = ray.o.z + t * ray.d.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.mat = self.mat.clone();
        rec.p = ray.point_at_parameter(t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        true
    }
    fn bounding_box(&self, _t0: f32, _t1: f32, b: &mut AABB) -> bool {
        *b = AABB::new();
        b.lower = Vec3::new(self.k - 0.0001, self.y0, self.z0);
        b.upper = Vec3::new(self.k + 0.0001, self.y1, self.z1);
        true
    }
}

pub struct BoxObject {
    pmin: Vec3,
    pmax: Vec3,
    list: HitList,
}
impl BoxObject {
    pub fn create(p0: Vec3, p1: Vec3, ptr: Rc<dyn Material>) -> Self {
        let mut walls = HitList::new();
        walls.list.push(Rc::new(XYRect {
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p1.z,
            mat: Some(ptr.clone()),
        }));
        walls.list.push(Rc::new(FlipNormals {
            ptr: Some(Rc::new(XYRect {
                x0: p0.x,
                x1: p1.x,
                y0: p0.y,
                y1: p1.y,
                k: p0.z,
                mat: Some(ptr.clone()),
            })),
        }));
        walls.list.push(Rc::new(XZRect {
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p1.y,
            mat: Some(ptr.clone()),
        }));
        walls.list.push(Rc::new(FlipNormals {
            ptr: Some(Rc::new(XZRect {
                x0: p0.x,
                x1: p1.x,
                z0: p0.z,
                z1: p1.z,
                k: p0.y,
                mat: Some(ptr.clone()),
            })),
        }));
        walls.list.push(Rc::new(YZRect {
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p1.x,
            mat: Some(ptr.clone()),
        }));
        walls.list.push(Rc::new(FlipNormals {
            ptr: Some(Rc::new(YZRect {
                y0: p0.y,
                y1: p1.y,
                z0: p0.z,
                z1: p1.z,
                k: p0.x,
                mat: Some(ptr.clone()),
            })),
        }));
        Self {
            pmin: p0,
            pmax: p1,
            list: walls,
        }
    }
}
impl Hitable for BoxObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        return self.list.hit(ray, t_min, t_max, rec);
    }
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool {
        *b = AABB::new();
        b.lower = self.pmin;
        b.upper = self.pmax;
        true
    }
}

pub struct FlipNormals {
    pub ptr: Option<Rc<dyn Hitable>>,
}
impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        match &self.ptr {
            None => panic!("No hitable object!"),
            Some(p) => {
                if p.hit(ray, t_min, t_max, rec) {
                    rec.normal = -rec.normal;
                    return true;
                }
            }
        }
        false
    }
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool {
        match &self.ptr {
            None => panic!("No hitable object!"),
            Some(p) => return p.bounding_box(t0, t1, b),
        }
    }
}

pub struct Translate {
    pub ptr : Option<Rc<dyn Hitable>>,
    pub offset : Vec3,
}
impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(ray.o - self.offset, ray.d, ray.time);
        match &self.ptr {
            None => panic!("No object!"),
            Some(p) => {
                if p.hit(&moved_r, t_min, t_max, rec) {
                    rec.p += self.offset;
                    return true;
                }
            }
        }
        false
    }
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool {
        match &self.ptr {
            None => panic!("No object!"),
            Some(p) => {
                if p.bounding_box(t0, t1, b) {
                    let mut aabb = AABB::new();
                    aabb.lower = self.offset + b.lower;
                    aabb.upper = self.offset + b.upper;
                    *b = aabb;
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub lower: Vec3,
    pub upper: Vec3,
}
impl AABB {
    fn new() -> Self {
        Self {
            lower: Vec3::new(0.0, 0.0, 0.0),
            upper: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1f32 / ray.d[a];
            let mut t0 = (self.lower[a] - ray.o[a]) * inv_d;
            let mut t1 = (self.upper[a] - ray.o[a]) * inv_d;
            if inv_d < 0f32 {
                swap(&mut t0, &mut t1);
            }
            let mut tmin = t_min;
            if t0 > t_min {
                tmin = t0;
            }
            let mut tmax = t_max;
            if t1 > t_max {
                tmax = t1;
            }
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    AABB {
        lower: Vec3::new(
            fmin(box0.lower.x, box1.lower.x),
            fmin(box0.lower.y, box1.lower.y),
            fmin(box0.lower.z, box1.lower.z),
        ),
        upper: Vec3::new(
            fmax(box0.upper.x, box1.upper.x),
            fmax(box0.upper.y, box1.upper.y),
            fmax(box0.upper.z, box1.upper.z),
        ),
    }
}

pub struct BVHNode {
    pub left: Option<Rc<dyn Hitable>>,
    pub right: Option<Rc<dyn Hitable>>,
    pub bbox: AABB,
}
impl BVHNode {
    pub fn new(l: &mut [Rc<dyn Hitable>], time0: f32, time1: f32) -> Self {
        let axis = (3.0 * rand()) as i32;
        if axis == 0 {
            l.sort_by(|a, b| {
                let mut box_left = AABB::new();
                let mut box_right = AABB::new();
                if a.bounding_box(0.0, 0.0, &mut box_left) == false
                    || b.bounding_box(0.0, 0.0, &mut box_right) == false
                {
                    println!("No bounding box in bvh_node constructor");
                }
                box_left.lower.x.partial_cmp(&box_right.lower.x).unwrap()
            });
        } else if axis == 1 {
            l.sort_by(|a, b| {
                let mut box_left = AABB::new();
                let mut box_right = AABB::new();
                if a.bounding_box(0.0, 0.0, &mut box_left) == false
                    || b.bounding_box(0.0, 0.0, &mut box_right) == false
                {
                    println!("No bounding box in bvh_node constructor");
                }
                box_left.lower.y.partial_cmp(&box_right.lower.y).unwrap()
            });
        } else {
            l.sort_by(|a, b| {
                let mut box_left = AABB::new();
                let mut box_right = AABB::new();
                if a.bounding_box(0.0, 0.0, &mut box_left) == false
                    || b.bounding_box(0.0, 0.0, &mut box_right) == false
                {
                    println!("No bounding box in bvh_node constructor");
                }
                box_left.lower.z.partial_cmp(&box_right.lower.z).unwrap()
            });
        }
        let n = l.len();
        let mut left = None;
        let mut right = None;
        if n == 1 {
            left = Some(l[0].clone());
            right = Some(l[0].clone());
        } else if n == 2 {
            left = Some(l[0].clone());
            right = Some(l[1].clone());
        } else {
            let n2 = n / 2;
            left = Some(Rc::new(BVHNode::new(&mut l[0..n2], time0, time1)));
            right = Some(Rc::new(BVHNode::new(&mut l[n2..n], time0, time1)));
        }
        let mut box_left = AABB::new();
        let mut box_right = AABB::new();
        match &left {
            None => println!("No bounding box in bvh_node constructor"),
            Some(left_node) => {
                if left_node.bounding_box(time0, time1, &mut box_left) == false {
                    println!("No bounding box in bvh_node constructor");
                }
            }
        }
        match &right {
            None => println!("No bounding box in bvh_node constructor"),
            Some(right_node) => {
                if right_node.bounding_box(time0, time1, &mut box_right) == false {
                    println!("No bounding box in bvh_node constructor");
                }
            }
        }
        Self {
            left,
            right,
            bbox: surrounding_box(&box_left, &box_right),
        }
    }
}
impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        if self.bbox.hit(ray, t_min, t_max) == true {
            let mut left_rec = HitRecord::new();
            let mut right_rec = HitRecord::new();
            let mut hit_left = false;
            match self.left {
                None => hit_left = false,
                Some(ref node) => {
                    hit_left = node.hit(ray, t_min, t_max, &mut left_rec);
                }
            }
            let mut hit_right = false;
            match self.right {
                None => hit_right = false,
                Some(ref node) => {
                    hit_right = node.hit(ray, t_min, t_max, &mut right_rec);
                }
            }
            if hit_left == true && hit_right == true {
                if left_rec.t < right_rec.t {
                    *rec = left_rec;
                } else {
                    *rec = right_rec;
                }
                return true;
            } else if hit_left == true {
                *rec = left_rec;
                return true;
            } else if hit_right == true {
                *rec = right_rec;
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, t0: f32, t1: f32, b: &mut AABB) -> bool {
        *b = self.bbox;
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::common::rand;
    use crate::core::hitable::*;
    trait ObjectTrait {
        fn method(&self) -> i32;
    }
    struct Object {
        pub x: i32,
        pub y: i32,
    }
    impl ObjectTrait for Object {
        fn method(&self) -> i32 {
            self.x
        }
    }
    #[test]
    fn sort_polymorphism() {
        let mut list: Vec<Box<dyn ObjectTrait>> = vec![];
        for i in 0..10 {
            list.push(Box::new(Object {
                x: (rand() * 100.0) as i32,
                y: i,
            }));
        }
        list.sort_by(|a, b| a.method().cmp(&b.method()));
        assert_eq!(list.len(), 10);
        for i in 0..9 {
            assert_eq!(list[i].method() <= list[i + 1].method(), true);
        }
    }
    #[test]
    fn sort_slices() {
        let mut list: Vec<Box<dyn ObjectTrait>> = vec![];
        for i in 0..10 {
            list.push(Box::new(Object {
                x: (rand() * 100.0) as i32,
                y: i,
            }));
        }
        let left = &mut list[0..6];
        left.sort_by(|a, b| a.method().cmp(&b.method()));
        let right = &mut list[6..10];
        right.sort_by(|a, b| a.method().cmp(&b.method()));
        for i in 0..5 {
            assert_eq!(list[i].method() <= list[i + 1].method(), true);
        }
        for i in 6..9 {
            assert_eq!(list[i].method() <= list[i + 1].method(), true);
        }
    }
    #[test]
    fn bvh_tree() {
        let mut world = HitList::new();
        world.list.push(Rc::new(SphereObject {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 0.3,
            mat: None,
        }));
        world.list.push(Rc::new(SphereObject {
            center: Vec3::new(0.0, 2.0, 0.0),
            radius: 0.3,
            mat: None,
        }));
        world.list.push(Rc::new(SphereObject {
            center: Vec3::new(0.0, 3.0, 0.0),
            radius: 0.3,
            mat: None,
        }));
        let bvh = BVHNode::new(&mut world.list[..], 0f32, 0f32);
        {
            let ray = Ray::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0);
            let mut rec = HitRecord::new();
            assert_eq!(bvh.hit(&ray, 0.0, 10.0, &mut rec), true);
        }
        {
            let ray = Ray::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0);
            let mut rec = HitRecord::new();
            assert_eq!(bvh.hit(&ray, 0.0, 10.0, &mut rec), true);
        }
        {
            let ray = Ray::new(Vec3::new(1.0, 3.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0);
            let mut rec = HitRecord::new();
            assert_eq!(bvh.hit(&ray, 0.0, 10.0, &mut rec), true);
        }
    }
}
