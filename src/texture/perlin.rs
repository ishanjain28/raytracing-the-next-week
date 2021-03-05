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
        let points = rng
            .sample_iter(Uniform::from(0.0..=1.0))
            .take(POINT_COUNT)
            .collect::<Vec<f64>>();

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

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut smooth_grid = [[[0.0; 2]; 2]; 2];

        for (di, a) in smooth_grid.iter_mut().enumerate() {
            let di = di as i32;
            for (dj, b) in a.iter_mut().enumerate() {
                let dj = dj as i32;
                for (dk, c) in b.iter_mut().enumerate() {
                    let dk = dk as i32;
                    *c = self.points[self.permute_x[((i + di) & 255) as usize]
                        ^ self.permute_y[((j + dj) & 255) as usize]
                        ^ self.permute_z[((k + dk) & 255) as usize]]
                }
            }
        }

        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        // Hermitian smoothing so we don't see obvious grid features in the picture
        // Those features show up when we interpolate colors. Those features are
        // also called mach bands
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        return trilinear_interpolate(smooth_grid, u, v, w);
    }
}

fn trilinear_interpolate(smooth_grid: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.0;

    for (di, a) in smooth_grid.iter().enumerate() {
        let di = di as f64;
        for (dj, b) in a.iter().enumerate() {
            let dj = dj as f64;
            for (dk, c) in b.iter().enumerate() {
                let dk = dk as f64;
                acc += (di * u + (1.0 - di) * (1.0 - u))
                    * (dj * v + (1.0 - dj) * (1.0 - v))
                    * (dk * w + (1.0 - dk) * (1.0 - w))
                    * c;
            }
        }
    }

    acc
}

fn permute<R: Rng + ?Sized>(rng: &mut R, p: &mut [usize]) {
    let l = p.len();

    for i in (0..l).rev() {
        let r = rng.gen_range(0..=i);
        p.swap(i, r);
    }
}
