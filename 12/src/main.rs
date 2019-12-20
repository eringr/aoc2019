
fn main() {
    let mut v = std::vec::Vec::<Body>::new();
    v.push(Body {
        pos: [-3,15,-11],
        vel: [0,0,0],
    });
    v.push(Body {
        pos: [3,13,-19],
        vel: [0,0,0],
    });
    v.push(Body {
        pos: [-13,18,-2],
        vel: [0,0,0],
    });
    v.push(Body {
        pos: [6,0,-1],
        vel: [0,0,0],
    });
    for b in v.iter() {
        println!("{:?}", b);
    }
    for n in 1..=1000 {
        println!("Step {}", n);
        simulate_step(&mut v);
    }

    let mut energy = 0;
    for b in v.iter() {
        let potential: i32 = b.pos.iter().map(|x| x.abs()).sum();
        let kinetic: i32 = b.vel.iter().map(|x| x.abs()).sum();
        energy += potential * kinetic;
    }
    println!("Total energy: {}", energy);
}

fn simulate_step(v: &mut std::vec::Vec::<Body>) {
    for i in 0..v.len() {
        for j in i+1..v.len() {
            for n in 0..3 {
                let (posl, posr) = (v[i].pos[n], v[j].pos[n]);
                if posl > posr {
                    v[i].vel[n] -= 1;
                    v[j].vel[n] += 1;
                } else if posl < posr {
                    v[i].vel[n] += 1;
                    v[j].vel[n] -= 1;
                }
            }
        }
    }
    for b in v.iter_mut() {
        for i in 0..3 {
            b.pos[i] += b.vel[i];
        }
        println!("{:?}", b);
    }
}

#[derive(Debug)]
struct Body {
    pos: [i32; 3],
    vel: [i32; 3],
}
