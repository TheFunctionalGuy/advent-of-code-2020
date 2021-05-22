use helper::lines_from_file;

fn main() {
    // Read tree map and parse to char vector
    let unparsed_tree_map = lines_from_file("src/bin/day_3/day_3.txt");
    let tree_map = create_tree_map(&unparsed_tree_map);

    // Directions for the first path
    let right = 3;
    let down = 1;

    // Calculate first path
    println!("Number of trees on the path with {} right and {} down: {}", right, down, count_trees_on_path(&tree_map, &right, &down));

    // Paths for second part
    let path_list = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    // Calculate path product
    let path_product: u32 = path_list.iter()
        .map(|(right, down)| count_trees_on_path(&tree_map, right, down))
        .product();

    println!("The product of trees on all paths is: {}", path_product);
}

// Converts the given tree map to 2d vector of chars
fn create_tree_map(unparsed_map: &Vec<String>) -> Vec<Vec<char>> {
    let tree_map = unparsed_map.iter()
        .map(|row| row.chars().collect())
        .collect();

    tree_map
}

// Counts trees on a path given by directions
fn count_trees_on_path(tree_map: &Vec<Vec<char>>, right: &usize, down: &usize) -> u32 {
    // Get dimensions
    let height = tree_map.len();
    let width = tree_map[0].len();

    let mut num_trees = 0;

    let mut position = (0, 0);

    while position.1 < height {
        if tree_map[position.1][position.0] == '#' {
            num_trees += 1
        }

        position.0 = (position.0 + right) % width;
        position.1 += down;
    }

    num_trees
}
