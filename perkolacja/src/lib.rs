use pyo3::prelude::*;

#[pyfunction]
fn perkhist() -> PyResult<([usize; 1000], [usize; 1000])> {
    const SAMPLE: usize = 1000;
    let mut l = [0; SAMPLE];
    let mut l2 = [0; SAMPLE];
    let normalfact = SAMPLE / 100;
    for i in 0..SAMPLE {
        l2[i] = i / normalfact;
        for _j in 0..50 {
            l[i] += perk_sq(i as f64 / SAMPLE as f64).unwrap() as usize
        }
        println!("{}", i);
    }
    Ok((l2, l))
}
#[pyfunction]
fn perk_sq(ratio: f64) -> PyResult<u8> {
    use rand::{seq::SliceRandom, thread_rng};
    const M: usize = 30;
    #[derive(Copy, Clone)]
    struct Forest {
        parent: usize,
        x: usize,
        rank: usize,
    }
    impl Forest {
        fn makenew(xn: usize) -> Forest {
            Forest {
                parent: xn,
                x: xn,
                rank: 0,
            }
        }
        fn find(&self, tree: [Forest; M.pow(2)]) -> Forest {
            let mut temp_parent = self.parent;
            let mut temp_x = self.x;
            // println!(temp_parent, temp_x)
            while temp_parent != temp_x {
                temp_x = tree[temp_x].parent;
                temp_parent = tree[temp_x].parent;
                // println!(temp_parent, temp_x)
            }
            // Zwraca "najstarszego przodka" instancji
            tree[temp_x]
        }

        fn union(x: Forest, y: Forest, mut tree: [Forest; M.pow(2)]) -> [Forest; M * M] {
            let mut x = x.find(tree);
            let mut y = y.find(tree);

            // x i y mają tą samą wartość, czyli prawdopodobnie są nawet tą samą instancją. Nie robimy nic
            if x.x == y.x {}
            // sortowanie według rangi - czy pierwszy, czy drugi argument zostanie rodzicem
            if x.rank < y.rank {
                (x, y) = (y, x)
            }
            // nie wiem, czy to potrzebne, ale nie chcę popsuć kodu. Jeśli instancja została wywołana funkcją union,
            // to rośnie jej ranga
            //x.rank += 1;
            //y.rank += 1;
            // po możliwej zamianie opisanej dwa komentarze wyżej ygrek przyjmuje iksa za rodzica
            y.parent = x.x;
            // funkcja jest jednokierunkowa, tu wymuszamy ten kierunek. Ma to większy sens w połączeniu
            // z kodem wywołującym funkcję union
            if x.rank == y.rank {
                x.rank = x.rank + 1;
            }
            tree[x.x] = x;
            tree[y.x] = y;
            tree
        }
    }
    /*fn Board() -> [[usize; M]; M] {
        let brd = [[0; M]; M];
        brd
    }*/
    /*fn pair_to_index(i: usize, j: usize, n: usize) -> usize {
        i + j * n
    }*/
    fn index_to_pair(id: usize) -> (usize, usize) {
        let j = id % M;
        let i = id / M;
        (i, j)
    }
    fn random_con(frac_n: usize, l: [usize; M.pow(2)]) -> Vec<usize> {
        let sampled_list: Vec<usize> = l
            .choose_multiple(&mut thread_rng(), frac_n)
            .cloned()
            .collect();
        sampled_list
    }

    //let bwrite = Board()
    let mut blist = [0; M.pow(2)];
    for j in 0..M.pow(2) {
        blist[j] = j;
    }

    let sampled_blist = random_con((ratio * (M.pow(2) as f64)) as usize, blist);

    // println!(sampled_blist)
    let mut first_row = Vec::new();
    let mut last_row = Vec::new();
    let mut trees = [Forest::makenew(0); M.pow(2) as usize];
    for &id in &sampled_blist {
        // (i, j) = index_to_pair(id, M)
        // bwrite[i][j] = 255
        trees[id] = Forest::makenew(id);
        if id < M {
            first_row.push(id);
            //
            // bez tego symulacja się partoli. Jest tu po to, żeby przypadkiem nie wyszło, że jakaś
            // instancja klasy odpowiadająca elementowi z rzędu i = 1 (czyli drugiego) nie miała
            // wyższej rangi niż instancja odpow. elementowi z rzędu i = 0 (pierwszego) i nie
            // "przyciągała" do siebie innych instancji jako "dzieci", zanim może to zrobić wartość z góry
            trees[id].rank = 10 * M;
        } else if id >= M * (M - 1) {
            last_row.push(id)
        }

        // println!(first_row)
        // println!(last_row)
    }

    fn sasiad(id: usize, id2: usize) -> bool {
        let (i, j) = index_to_pair(id);
        let (k, l) = index_to_pair(id2);
        match (i as i64 - k as i64, j as i64 - l as i64) {
            (-1, 0) => true,
            (1, 0) => true,
            (0, -1) => true,
            (0, 1) => true,
            _ => false,
        }
    }
    //sampled_blist.sort();
    for &id in &sampled_blist {
        for &id2 in &sampled_blist {
            if id < id2 && sasiad(id, id2) {
                trees = Forest::union(trees[id], trees[id2], trees);
            }
        }
    }
    for &id2 in &last_row {
        for &id in &first_row {
            if id == trees[id2].find(trees).x {
                // println!("Perkolacja!")
                return Ok(1);
            }
        }
    }
    Ok(0)
}
#[pyfunction]
fn perk_t(ratio: f64) -> PyResult<u8> {
    use rand::{seq::SliceRandom, thread_rng};
    const M: usize = 40;
    #[derive(Copy, Clone)]
    struct Forest {
        parent: usize,
        x: usize,
        rank: usize,
    }
    impl Forest {
        fn makenew(xn: usize) -> Forest {
            Forest {
                parent: xn,
                x: xn,
                rank: 0,
            }
        }
        fn find(&self, tree: [Forest; M.pow(2)]) -> Forest {
            let mut temp_parent = self.parent;
            let mut temp_x = self.x;
            // println!(temp_parent, temp_x)
            while temp_parent != temp_x {
                temp_x = tree[temp_x].parent;
                temp_parent = tree[temp_x].parent;
                // println!(temp_parent, temp_x)
            }
            // Zwraca "najstarszego przodka" instancji
            tree[temp_x]
        }

        fn union(x: Forest, y: Forest, mut tree: [Forest; M.pow(2)]) -> [Forest; M * M] {
            let mut x = x.find(tree);
            let mut y = y.find(tree);

            // x i y mają tą samą wartość, czyli prawdopodobnie są nawet tą samą instancją. Nie robimy nic
            if x.x == y.x {}
            // sortowanie według rangi - czy pierwszy, czy drugi argument zostanie rodzicem
            if x.rank < y.rank {
                (x, y) = (y, x)
            }
            // nie wiem, czy to potrzebne, ale nie chcę popsuć kodu. Jeśli instancja została wywołana funkcją union,
            // to rośnie jej ranga
            x.rank += 1;
            y.rank += 1;
            // po możliwej zamianie opisanej dwa komentarze wyżej ygrek przyjmuje iksa za rodzica
            y.parent = x.x;
            // funkcja jest jednokierunkowa, tu wymuszamy ten kierunek. Ma to większy sens w połączeniu
            // z kodem wywołującym funkcję union
            if x.rank == y.rank {
                x.rank = x.rank + 1;
            }
            tree[x.x] = x;
            tree[y.x] = y;
            tree
        }
    }
    /*fn Board() -> [[usize; M]; M] {
        let brd = [[0; M]; M];
        brd
    }*/
    /*fn pair_to_index(i: usize, j: usize, n: usize) -> usize {
        i + j * n
    }*/
    fn index_to_pair(id: usize) -> (usize, usize) {
        let j = id % M;
        let i = id / M;
        (i, j)
    }
    fn random_con(frac_n: usize, l: [usize; M.pow(2)]) -> Vec<usize> {
        let mut templ = Vec::new();
        for id in l {
            let (i, j) = index_to_pair(id);
            if i % 2 == 0 {
                templ.push(id)
            } else if j != 0 {
                templ.push(id)
            }
        }
        let sampled_list: Vec<usize> = templ
            .choose_multiple(&mut thread_rng(), frac_n)
            .cloned()
            .collect();
        sampled_list
    }

    //let bwrite = Board()
    let mut blist = [0; M.pow(2)];
    for j in 0..M.pow(2) {
        blist[j] = j;
    }

    let mut sampled_blist = random_con((ratio * ((M.pow(2) - M / 2) as f64)) as usize, blist);

    // println!(sampled_blist)
    let mut first_row = Vec::new();
    let mut last_row = Vec::new();
    let mut trees = [Forest::makenew(0); M.pow(2) as usize];
    for &id in &sampled_blist {
        // (i, j) = index_to_pair(id, M)
        // bwrite[i][j] = 255
        trees[id] = Forest::makenew(id);
        if id < M {
            first_row.push(id);
            //
            // bez tego symulacja się partoli. Jest tu po to, żeby przypadkiem nie wyszło, że jakaś
            // instancja klasy odpowiadająca elementowi z rzędu i = 1 (czyli drugiego) nie miała
            // wyższej rangi niż instancja odpow. elementowi z rzędu i = 0 (pierwszego) i nie
            // "przyciągała" do siebie innych instancji jako "dzieci", zanim może to zrobić wartość z góry
            trees[id].rank = 10 * M;
        } else if id >= M * (M - 1) {
            last_row.push(id)
        }

        // println!(first_row)
        // println!(last_row)
    }

    fn sasiad(id: usize, id2: usize) -> bool {
        let (i, j) = index_to_pair(id);
        let (k, l) = index_to_pair(id2);
        match (
            i % 2,
            k % 2,
            i % 3,
            k % 3,
            j,
            l,
            i as i64 - k as i64,
            j as i64 - l as i64,
        ) {
            (1, _, _, _, 0, _, _, _) => false,
            (_, 1, _, _, _, 0, _, _) => false,
            (_, _, _, _, _, _, 0, 1) => true,
            (_, _, _, _, _, _, 0, -1) => true,
            (_, _, _, _, _, _, 1, 0) => true,
            (_, _, _, _, _, _, -1, 0) => true,
            (_, _, 0, _, _, _, -1, -1) => true,
            (_, _, _, 0, _, _, 1, 1) => true,
            (_, _, 2, _, _, _, 1, -1) => true,
            (_, _, _, 2, _, _, -1, 1) => true,
            _ => false,
        }
    }
    sampled_blist.sort();
    for &id in &sampled_blist {
        for &id2 in &sampled_blist {
            if id < id2 && sasiad(id, id2) {
                trees = Forest::union(trees[id], trees[id2], trees);
            }
        }
    }

    for &id2 in &last_row {
        for &id in &first_row {
            if id == trees[id2].find(trees).x {
                // println!("Perkolacja!")
                return Ok(1);
            }
        }
    }
    Ok(0)
}
#[pyfunction]
fn perk_hex(ratio: f64) -> PyResult<u8> {
    use rand::{seq::SliceRandom, thread_rng};
    const M: usize = 50;
    #[derive(Copy, Clone)]
    struct Forest {
        parent: usize,
        x: usize,
        rank: usize,
    }
    impl Forest {
        fn makenew(xn: usize) -> Forest {
            Forest {
                parent: xn,
                x: xn,
                rank: 0,
            }
        }
        fn find(&self, tree: [Forest; M.pow(2)]) -> Forest {
            let mut temp_parent = self.parent;
            let mut temp_x = self.x;
            // println!(temp_parent, temp_x)
            while temp_parent != temp_x {
                temp_x = tree[temp_x].parent;
                temp_parent = tree[temp_x].parent;
                // println!(temp_parent, temp_x)
            }
            // Zwraca "najstarszego przodka" instancji
            tree[temp_x]
        }

        fn union(x: Forest, y: Forest, mut tree: [Forest; M.pow(2)]) -> [Forest; M * M] {
            let mut x = x.find(tree);
            let mut y = y.find(tree);

            // x i y mają tą samą wartość, czyli prawdopodobnie są nawet tą samą instancją. Nie robimy nic
            if x.x == y.x {}
            // sortowanie według rangi - czy pierwszy, czy drugi argument zostanie rodzicem
            if x.rank < y.rank {
                (x, y) = (y, x)
            }
            // nie wiem, czy to potrzebne, ale nie chcę popsuć kodu. Jeśli instancja została wywołana funkcją union,
            // to rośnie jej ranga
            x.rank += 1;
            y.rank += 1;
            // po możliwej zamianie opisanej dwa komentarze wyżej ygrek przyjmuje iksa za rodzica
            y.parent = x.x;
            // funkcja jest jednokierunkowa, tu wymuszamy ten kierunek. Ma to większy sens w połączeniu
            // z kodem wywołującym funkcję union
            if x.rank == y.rank {
                x.rank = x.rank + 1;
            }
            tree[x.x] = x;
            tree[y.x] = y;
            tree
        }
    }
    /*fn Board() -> [[usize; M]; M] {
        let brd = [[0; M]; M];
        brd
    }*/
    /*fn pair_to_index(i: usize, j: usize, n: usize) -> usize {
        i + j * n
    }*/
    fn index_to_pair(id: usize) -> (usize, usize) {
        let j = id % M;
        let i = id / M;
        (i, j)
    }
    fn random_con(frac_n: usize, l: [usize; M.pow(2)]) -> Vec<usize> {
        let sampled_list: Vec<usize> = l
            .choose_multiple(&mut thread_rng(), frac_n)
            .cloned()
            .collect();
        sampled_list
    }

    //let bwrite = Board()
    let mut blist = [0; M.pow(2)];
    for j in 0..M.pow(2) {
        blist[j] = j;
    }

    let mut sampled_blist = random_con((ratio * (M.pow(2) as f64)) as usize, blist);

    // println!(sampled_blist)
    let mut first_row = Vec::new();
    let mut last_row = Vec::new();
    let mut trees = [Forest::makenew(0); M.pow(2) as usize];
    for &id in &sampled_blist {
        // (i, j) = index_to_pair(id, M)
        // bwrite[i][j] = 255
        trees[id] = Forest::makenew(id);
        if id < M {
            first_row.push(id);
            //
            // bez tego symulacja się partoli. Jest tu po to, żeby przypadkiem nie wyszło, że jakaś
            // instancja klasy odpowiadająca elementowi z rzędu i = 1 (czyli drugiego) nie miała
            // wyższej rangi niż instancja odpow. elementowi z rzędu i = 0 (pierwszego) i nie
            // "przyciągała" do siebie innych instancji jako "dzieci", zanim może to zrobić wartość z góry
            trees[id].rank = 10 * M;
        } else if id >= M * (M - 1) {
            last_row.push(id)
        }

        // println!(first_row)
        // println!(last_row)
    }

    fn sasiad(id: usize, id2: usize) -> bool {
        let (i, j) = index_to_pair(id);
        let (k, l) = index_to_pair(id2);
        match (
            i as i64 - k as i64,
            j as i64 - l as i64,
            i % 2,
            k % 2,
            l % 2,
        ) {
            (1, 0, ..) => true,
            (-1, 0, ..) => true,
            (0, 1, 0, 1, 0) => true,
            (0, -1, 0, 0, 1) => true,
            (0, 1, 1, 0, 1) => true,
            (0, -1, 1, 1, 0) => true,
            _ => false,
        }
    }
    sampled_blist.sort();
    for &id in &sampled_blist {
        for &id2 in &sampled_blist {
            if id < id2 && sasiad(id, id2) {
                trees = Forest::union(trees[id], trees[id2], trees);
            }
        }
    }

    for &id2 in &last_row {
        for &id in &first_row {
            if id == trees[id2].find(trees).x {
                // println!("Perkolacja!")
                return Ok(1);
            }
        }
    }
    Ok(0)
}

/// A Python module implemented in Rust.
#[pymodule]
fn perkolacja(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(perk_sq, m)?)?;
    m.add_function(wrap_pyfunction!(perkhist, m)?)?;
    m.add_function(wrap_pyfunction!(perk_t, m)?)?;
    m.add_function(wrap_pyfunction!(perk_hex, m)?)?;
    Ok(())
}
