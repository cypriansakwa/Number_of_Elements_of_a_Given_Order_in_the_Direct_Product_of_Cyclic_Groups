## Overview
This Rust program calculates the number of elements of a specific order in the direct product of two cyclic groups, $\mathbb{Z}_n\times\mathbb{Z}_m$. The program uses basic group theory concepts like Euler's Totient function and the least common multiple (LCM) to compute the order of elements in product groups.
## Features
- Computes the greatest common divisor (GCD) and least common multiple (LCM) of two numbers.
- Uses Euler's Totient function to find the number of elements of a specific order in a cyclic group $\mathbb{Z}_n$.
- Finds the number of elements of a given order in the product of two cyclic groups $\mathbb{Z}_n\times\mathbb{Z}_m$.
## Example
- To calculate the number of elements of order 7 in $\mathbb{Z}_{49}\times\mathbb{Z}_{14}$, run the program with the following parameters:
>```
>fn main() {
>let n = 49;
>let m = 14;
>let order = 7;
> let num_elements = num_elements_of_order_in_product_group(n, m, order);
>println!("The number of elements of order {} in Z_{} x Z_{} is: {}", order, n, m, num_elements);
>}
- The output will be:
 >```
> The number of elements of order 7 in Z_49 x Z_14 is: 48
## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone or download this repository.
- Compile and run the Rust program using the following command:
>```
>cargo build
>cargo run
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/Number_of_Elements_of_a_Given_Order_in_the_Direct_Product_of_Cyclic_Groups.git
   cd Number_of_Elements_of_a_Given_Order_in_the_Direct_Product_of_Cyclic_Groups
