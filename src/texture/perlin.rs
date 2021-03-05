use crate::types::Vec3;
use rand::{distributions::Uniform, Rng};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    points: Vec<f64>,

    permute_x: Vec<usize>,
    permute_y: Vec<usize>,
    permute_z: Vec<usize>,
}

impl Perlin {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut points = vec![0.0; POINT_COUNT];

        for p in points.iter_mut() {
            *p = rng.gen_range(0.0..=1.0)
        }

        let permute_x = Self::perlin_generate_permutation(rng);
        let permute_y = Self::perlin_generate_permutation(rng);
        let permute_z = Self::perlin_generate_permutation(rng);

        Self {
            points,
            permute_x,
            permute_y,
            permute_z,
        }
    }

    fn perlin_generate_permutation<R: Rng + ?Sized>(rng: &mut R) -> Vec<usize> {
        let mut p = (0..POINT_COUNT).collect::<Vec<usize>>();
        permute(rng, &mut p);
        p
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = (4.0 * p.x()) as usize & 255;
        let j = (4.0 * p.y()) as usize & 255;
        let k = (4.0 * p.z()) as usize & 255;

        //        if p.x() > 1.0 && p.y() > 1.0 && p.z() > 1.0 {
        //            println!(
        //                "p = {} i = {} j = {} k = {} pi = {} pj = {} pk = {} point = {}",
        //                p,
        //                i,
        //                j,
        //                k,
        //                self.permute_x[i],
        //                self.permute_x[j],
        //                self.permute_x[k],
        //                self.points[self.permute_x[i] ^ self.permute_y[j] ^ self.permute_z[k]]
        //            );
        //        }

        self.points[self.permute_x[i] ^ self.permute_y[j] ^ self.permute_z[k]]
    }
}

fn permute<R: Rng + ?Sized>(rng: &mut R, p: &mut [usize]) {
    let l = p.len();

    for i in (0..l).rev() {
        let r = rng.gen_range(0..=i);
        p.swap(i, r);
    }
}
