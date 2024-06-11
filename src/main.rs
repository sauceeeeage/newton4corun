use std::hint::spin_loop;

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
    // seems like only fit some cases
    let mut ris: Vec<i32> = Vec::new();
    // let base = 2 * (m - 1);
    assert!((c2 as u32).is_power_of_two(), "c2 must be power of 2");
    let largest_ri = 2 * (m - c2);
    println!("largest_ri: {}", largest_ri);
    for _ in 0..c2 {
        ris.push(largest_ri);
    } // generate the bottom half of the sawtooth

    for i in ((c2 + 1)..).step_by(2).take((m - 2 * c2) as usize) {
        // println!("i: {}", i);
        ris.push(i);
    }
    ris
}

fn old_generate_ris(m: i32, c2: i32) -> Vec<i32> {
    let mut ris: Vec<i32> = Vec::new();

    for i in 1..=m - c2 {
        ris.push(2 * i);
    }
    ris
}

fn main() {
    let delta = 0.0001;
    let mut mr: f32 = 0.75;
    let m = 128 * MI / 8;
    let c2 = 1 * MI / 8;
    let c3 = 96 * MI / 8;
    // let ris = generate_ris(m, c2);
    // let ris = old_generate_ris(m, c2);
    // println!("{:?}", ris);
    loop {
        // let newton_mr = sawtooth_newton(mr, m, c2, c3, &ris);
        let newton_mr = cyclic_newton(mr, m, c2, c3);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ris() {
        let m = 16;
        let c2 = 4;
        let ris = generate_ris(m, c2);
        println!("{:?}", ris);
        assert_eq!(ris.len(), 12);
    }

    #[test]
    fn test_cyclic_newton() {
        let mr = 0.95;
        let m = 1024 * MI;
        let c2 = 1 * MI;
        let c3 = 96 * MI;
        let newton_mr = cyclic_newton(mr, m, c2, c3);
        assert_eq!(newton_mr, 0.9999);
    }

    #[test]
    fn test_sawtooth_newton() {
        let mr = 0.95;
        let m = 1024 * MI;
        let c2 = 1 * MI;
        let c3 = 96 * MI;
        let ris = generate_ris(m, c2);
        let newton_mr = sawtooth_newton(mr, m, c2, c3, &ris);
        assert_eq!(newton_mr, 0.98435974);
    }
}

/* model results */

// cyclic with original cyclic formula with Byte granularity
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.445), (256, 0.911), (512, 0.995), (1024, 0.9999)]

// cyclic with original cyclic formula with block granularity
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.014), (256, 0.797), (512, 0.980), (1024, 0.9996)]

// sawtooth with original sawtooth ris generation with Byte granularity
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.342), (256, 0.755), (512, 0.969), (1024, 0.9844)]

// sawtooth with original sawtooth ris generation with block granularity
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.496), (256, 0.749), (512, 0.879), (1024, 0.9439)]

// sawtooth with new sawtooth ris generation with block granularity
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.496), (256, 0.749), (512, 0.881), (1024, 0.9445)]

// sawtooth with new sawtooth ris generation with Byte granularity
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.874), (256, 0.937), (512, 0.969), (1024, 0.9844)]
