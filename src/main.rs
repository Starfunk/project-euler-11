/*My solution to problem 11 from projecteuler.com

Q: What is the greatest product of four adjacent numbers in the same direction
(up, down, left, right, or diagonally) in the 20×20 grid?

I employ a simplistic brute force method, looping through every row, column, right diagonal, and
left diagonal 4-number product to find the largest product given these combinations.*/

//Define struct that records value and location of each position on the grid
struct GridPosition {
    value: usize,
    row: usize,
    column: usize,
}

/*Turns each element in data_set into a GridPosition, recording value, row, and column information.
Outputs a vector where each element is a GridPosition. This is my way of creating a "2D" grid
vector in one dimension.*/
fn mapping(arr: &[usize] ) -> Vec<GridPosition> {
    let mut grid: Vec<GridPosition> = Vec::new();
    let mut counter = 0;

    for i in 1..21 {
        for j in 1..21 {
            let mut position_piece = GridPosition {
                value: arr[counter],
                row: i,
                column: j,
            };
            grid.push(position_piece);
            counter += 1;
        }
    }
    grid
}

//Defining useful functions for GridPosition
impl GridPosition {
    fn get_value(&self) -> usize {
        self.value
    }
    fn get_row(&self) -> usize {
        self.row
    }
    fn get_column(&self) -> usize {
        self.column
    }
    //Creates a clone of a GridPosition object
    fn clone(&self) -> GridPosition {
        let clone_self = GridPosition {
            value: self.get_value(),
            row: self.get_row(),
            column: self.get_column(),
        };
        clone_self
    }
}

//This function is responsible for looping through every possible 4-number row product.
fn row_test(v: Vec<GridPosition>) -> Vec<GridPosition> {
    let mut product: usize = 0;
    let mut row_0 = 0;
    let mut row_1 = 1;
    let mut row_2 = 2;
    let mut row_3 = 3;
    let mut return_vector: Vec<GridPosition> = Vec::new();

    for i in 0..20 {
        for j in 0..16 {
            if product <= &v[row_0].get_value() * &v[row_1].get_value() *
            &v[row_2].get_value() * &v[row_3].get_value() {
                product = &v[row_0].get_value() * &v[row_1].get_value() *
                &v[row_2].get_value() * &v[row_3].get_value();

                //We only want to return the largest 4-number product. Therefore, we clear the
                //vector everytime we find a larger 4-number product.
                return_vector.clear();

                //We need to clone the pushed elements, or else we will receive a "cannot move out
                //of indexed content" error.
                return_vector.push(v[row_0].clone());
                return_vector.push(v[row_1].clone());
                return_vector.push(v[row_2].clone());
                return_vector.push(v[row_3].clone());
            }
            row_0 += 1;
            row_1 += 1;
            row_2 += 1;
            row_3 += 1;
        }
    }
    return_vector
}

//This function is responsible for looping through every possible 4-number column product.
fn column_test(v: Vec<GridPosition>) -> Vec<GridPosition> {
    let mut product: usize = 0;

    let mut return_vector: Vec<GridPosition> = Vec::new();

    for i in 0..20 {
        let mut column_0 = i;
        let mut column_1 = i + 20;
        let mut column_2 = i + 40;
        let mut column_3 = i + 60;

        for j in 0..16 {
            if product <= v[column_0].get_value() * v[column_1].get_value() *
            v[column_2].get_value() * v[column_3].get_value() {
                product = v[column_0].get_value() * v[column_1].get_value() *
                v[column_2].get_value() * v[column_3].get_value();

                return_vector.clear();
                return_vector.push(v[column_0].clone());
                return_vector.push(v[column_1].clone());
                return_vector.push(v[column_2].clone());
                return_vector.push(v[column_3].clone());
            }
            column_0 += 20;
            column_1 += 20;
            column_2 += 20;
            column_3 += 20;
        }
    }
    return_vector
}

//This function is responsible for looping through every possible 4-number diagonal product
//where the diagonal points down and to the right.
fn right_diagonal_test(v: Vec<GridPosition>) -> Vec<GridPosition> {
    let mut product: usize = 0;

    let mut return_vector: Vec<GridPosition> = Vec::new();

    for i in 0..16 {
        let mut r_diag_0 = i * 20;
        let mut r_diag_1 =  (i * 20) + 21;
        let mut r_diag_2 = (i * 20) + 42;
        let mut r_diag_3 = (i * 20) + 63;

        for j in 0..16 {

            if product <= v[r_diag_0].get_value() * v[r_diag_1].get_value() *
            v[r_diag_2].get_value() * v[r_diag_3].get_value() {
                product = v[r_diag_0].get_value() * v[r_diag_1].get_value() *
                v[r_diag_2].get_value() * v[r_diag_3].get_value();

                return_vector.clear();

                return_vector.push(v[r_diag_0].clone());
                return_vector.push(v[r_diag_1].clone());
                return_vector.push(v[r_diag_2].clone());
                return_vector.push(v[r_diag_3].clone());
            }
            r_diag_0 += 1;
            r_diag_1 += 1;
            r_diag_2 += 1;
            r_diag_3 += 1;
        }

    }
    return_vector
}

//This function is responsible for looping through every possible 4-number diagonal product
//where the diagonal points down and to the left.
fn left_diagonal_test(v: Vec<GridPosition>) -> Vec<GridPosition> {
    let mut product: usize = 0;
    let mut return_vector: Vec<GridPosition> = Vec::new();
    let mut counter = 0;

    for i in 3..20 {
        let mut l_diag_0 = 3 + 20 * counter;
        let mut l_diag_1 =  22 + 20 * counter;
        let mut l_diag_2 = 41 + 20 * counter;
        let mut l_diag_3 = 60 + 20 * counter;

        for j in 3..20 {
            if product <= v[l_diag_0].get_value() * v[l_diag_1].get_value() *
            v[l_diag_2].get_value() * v[l_diag_3].get_value() {

                product = v[l_diag_0].get_value() * v[l_diag_1].get_value() *
                v[l_diag_2].get_value() * v[l_diag_3].get_value();

                return_vector.clear();

                return_vector.push(v[l_diag_0].clone());
                return_vector.push(v[l_diag_1].clone());
                return_vector.push(v[l_diag_2].clone());
                return_vector.push(v[l_diag_3].clone());
            }
            l_diag_0 += 1;
            l_diag_1 += 1;
            l_diag_2 += 1;
            l_diag_3 += 1;
        }
        counter += 1;
    }
    return_vector
}

//Prints the position and value of each element in a Vec<GridPosition> and returns the product
//of the values.
fn get_product_result(v: Vec<GridPosition>) -> usize {
    let mut product = 1;

    for position_piece in v {

        println!("Value: {}, Row: {}, Column: {} ", position_piece.get_value(),
        position_piece.get_row(), position_piece.get_column());

        product *= position_piece.get_value();
    }
    product
}

fn main() {

    //The data set is provided by projecteuler
    let data_set  =
   [08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
    49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
    81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
    52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
    22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
    24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
    78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
    16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
    86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
    04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
    20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
    20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
    01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48];

    //Create 4 identical "2D" vectors of data_set where each element of each vector is a GridPosition.
    let grid_0 = mapping(&data_set);
    let grid_1 = mapping(&data_set);
    let grid_2 = mapping(&data_set);
    let grid_3 = mapping(&data_set);

    /*Store the largest 4-number product of every row, column, right diagonal, and left diagonal
    combination in the variables: product_0, product_1, product_2, and product_3 respectively.
    The variables are of type Vec<GridPosition>.*/
    let product_0 = row_test(grid_0);
    let product_1 = column_test(grid_1);
    let product_2 = right_diagonal_test(grid_2);
    let product_3 = left_diagonal_test(grid_3);

    //Lastly, print out the positions and values of each element in the 4 vectors. And print the
    //the product so that we can compare them all together and see which one is the largest.

    println!("The positions of the maximum row product are:");
    println!("The maximum row product is: {}
    ", get_product_result(product_0));

    println!("The positions of the maximum column product are:");
    println!("The maximum column product is: {}
    ", get_product_result(product_1));

    println!("The positions of the maximum right diagonal product are:");
    println!("The maximum right diagonal product is: {}
    ", get_product_result(product_2));

    println!("The positions of the maximum left diagonal product are:");
    println!("The maximum left diagonal product is: {}",
    get_product_result(product_3));
    
}
