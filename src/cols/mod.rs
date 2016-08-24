use fs::file::File;
use super::NTHREADS;

pub fn to_sep_col<'a>(col: &mut Vec<File>) -> Vec<Vec<File>> {
    let x = col.len() as f32 / (NTHREADS as f32);
    let (extra, iterations) = (((x % 1.0) * (NTHREADS as f32)) as i32, x as i32);
    let mut vec = vec![iterations; NTHREADS as usize];

    for i in 0..extra {
        vec[i as usize] = iterations + 1
    }

    let mut sliced: Vec<Vec<File>> = Vec::new();
    for i in 0..NTHREADS {
        let iter = vec[i as usize];
        let mut inner_vec: Vec<File> = Vec::new();
        for _ in 0..iter {
            let file: File = col.pop().unwrap();
            inner_vec.push(file);
        }
        sliced.push(inner_vec);
    }

    sliced
}
