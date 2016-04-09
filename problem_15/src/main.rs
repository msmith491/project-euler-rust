fn main() {

    let vec = vec![20, 20];

    fn path(grid: Vec<u64>) -> u64 {

        let mut paths = 0;

        if grid == vec![0, 0] {

            return 1;
        }

        if grid[0] != 0 {

            paths = paths + path(vec![grid[0] - 1, grid[1]]);

        }

        if grid[1] != 0 {

            paths = paths + path(vec![grid[0], grid[1] - 1]);

        }

        return paths;
    }

    fn path_iter(size: usize) -> usize {

        let mut vec: Vec<usize> = vec![1; size + 1];
        let slice = vec.as_mut_slice();

        for i in 1..(size + 1) {
            for j in 1..i {
                slice[j] = slice[j] + slice[j-1];
            }
            slice[i] = 2 * slice[i - 1];
        }
        // println!("{:?}", slice);
        slice[size]
    }

    println!("{}", path_iter(20));
}
