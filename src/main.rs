const KI: i32 = 1024;
const MI: i32 = 1024 * KI;

fn cyclic_newton(mr: f32, m: i32, c2: i32, c3: i32) -> f32 {
    let power = m as f32 * mr;
    let base = 1. - (1. / c3 as f32);
    let hit_ratio = base.powf(power);
    let miss_ratio = 1. - hit_ratio;
    miss_ratio
}

fn sawtooth_newton(mr: f32, m: i32, c2: i32, c3: i32, ris: &Vec<i32>) -> f32 {
    let mut sum: f32 = 0.;

    for ri in ris {
        sum += (1. - (1. / c3 as f32)).powf(*ri as f32 * mr);
    }

    let hit_ratio = sum / (m as f32 - c2 as f32);
    let miss_ratio = 1. - hit_ratio;
    miss_ratio
}

fn generate_ris(m: i32, c2: i32) -> Vec<i32> {
    let mut ris: Vec<i32> = Vec::new();

    for i in 1..=m - c2 {
        ris.push(2 * i);
    }

    ris.sort();
    ris.dedup();
    ris
}

fn main() {
    let delta = 0.0001;
    let mut mr = 0.95;
    let m = 1024 * MI;
    let c2 = 1 * MI;
    let c3 = 96 * MI;
    let ris = generate_ris(m, c2);
    while true {
        let newton_mr = sawtooth_newton(mr, m, c2, c3, &ris);
        // let newton_mr = cyclic_newton(mr, m, c2, c3);
        let diff = (mr - newton_mr).abs();

        if diff < delta {
            println!("final mr: {}", newton_mr);
            break;
        }

        println!("mr: {}, newton_mr: {}, diff: {}", mr, newton_mr, diff);

        if newton_mr > mr {
            mr += f32::max(diff / 2., delta);
        } else {
            mr -= f32::max(diff / 2., delta);
        }
    }
}

// cyclic with original cyclic formula
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.445), (256, 0.911), (512, 0.995), (1024, 0.9999)]

// sawtooth with new sawtooth formula
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.34164199234095116), (256, 0.7551754414544363), (512, 0.96868885), (1024, 0.98435974)]
