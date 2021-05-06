fn main() {
    let a: [[u8; 2]; 3] = [[1, 2],
                           [3, 4],
                           [5, 6]];
    let b: [[u8; 5]; 2] = [[1, 2, 3, 4, 5],
                           [4, 5, 6, 7, 8]];


    for i in 0..a.len() {
        for j in 0..b[0].len() {
            let mut res = 0;
            for k in 0..a[0].len() {
                res += a[i][k] * b[k][j];
            }

            println!("{}", res);
        }
        println!("new line");
    }
}
