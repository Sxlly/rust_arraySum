// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 18/09/24


fn main() { //declare main funciton 

    let array: [i32; 5] = [10,20,30,40,50]; //declare array with all contents having a 32 bit signed integer data type and 5 index length

    let arraysum: i32 = array.iter().sum(); // decalre arraysum variable as 32 bit signed integer -> use array iter method to iterate through array and sum method to get sum

    println!( //print to terminal sum of array filling {} -> arraysum variable

        "sum of array is {}",
        arraysum //fill arraysum -> {}
    );

}