fn gen_table(w: usize, h: usize) -> Vec<Vec<usize>> {
    let mut t = vec![vec![0; h]; w];

    let mut game_value_array = vec![false; w + h + 1];

    // 1x
    for x in 0..w {
        t[x][0] = x + 1;
        t[x][1] = x + 2;
    }
    for y in 0..h {
        t[0][y] = y + 1;
        t[1][y] = y + 2;
    }

    for x in 2..w {
        for y in x..h {
            for i in 0..game_value_array.len() {
                game_value_array[i] = false;
            }

            for i in 0..(x + y + 1) {
                if i < y {
                    let up = t[x][y - i - 1];
                    game_value_array[up] = true;
                    game_value_array[i ^ x] = true;
                    game_value_array[i ^ up] = true;
                } else if i == y {
                    game_value_array[x] = true;
                    game_value_array[y] = true;
                    game_value_array[x ^ y] = true;
                } else {
                    let dominoes_to_left = i - y; // index - (height - 1)
                    let dominoes_to_right = x - dominoes_to_left; // (width - 1) - dl
                    game_value_array[y ^ dominoes_to_right] = true;
                    let right = t[y][dominoes_to_left - 1];
                    game_value_array[right] = true;
                    game_value_array[right ^ dominoes_to_right] = true;
                }
            }

            let mut mex = 0;

            for i in 1..game_value_array.len() {
                if !game_value_array[i] {
                    mex = i;
                    break;
                }
            }

            t[x][y] = mex;
            t[y][x] = mex;
        }
    }

    t
}

fn main() {
    let t = gen_table(2000, 2000);
    std::fs::write("data.json", serde_json::to_string(&t).unwrap()).unwrap();
}
