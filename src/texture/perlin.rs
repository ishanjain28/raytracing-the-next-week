use crate::types::Vec3;
use rand::Rng;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    points: Vec<Vec3>,

    permute_x: Vec<usize>,
    permute_y: Vec<usize>,
    permute_z: Vec<usize>,
}

impl Perlin {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let points = (0..POINT_COUNT)
            .map(|_| Vec3::random(rng).unit_vector())
            .collect::<Vec<Vec3>>();

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
        let mut smooth_grid = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        {
            let i = p.x().floor() as i32;
            let j = p.y().floor() as i32;
            let k = p.z().floor() as i32;
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
        }

        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        perlin_interpolate(smooth_grid, u, v, w)
    }

    pub fn turbulence(&self, p: Vec3, depth: u32) -> f64 {
        let mut acc = 0.0f64;
        let mut weight = 1.0;
        let mut temp_p = p;

        for _i in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }
}

fn perlin_interpolate(smooth_grid: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // Hermitian smoothing so we don't see obvious grid features in the picture
    // Those features show up when we interpolate colors. Those features are
    // also called mach bands
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;

    for (di, a) in smooth_grid.iter().enumerate() {
        let di = di as f64;
        for (dj, b) in a.iter().enumerate() {
            let dj = dj as f64;
            for (dk, c) in b.iter().enumerate() {
                let dk = dk as f64;

                let wt = Vec3::new(u - di, v - dj, w - dk);

                acc += (di * uu + (1.0 - di) * (1.0 - uu))
                    * (dj * vv + (1.0 - dj) * (1.0 - vv))
                    * (dk * ww + (1.0 - dk) * (1.0 - ww))
                    * c.dot(&wt);
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
