mod maths;

use spl_math::precise_number::PreciseNumber;
use wasm_bindgen::prelude::*;
use crate::maths::sqrt_precise;

const MIN_LIQUIDITY: u64 = 100;

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


pub fn calculate_first_deposit_lp_tokens_to_mint(
    token_a_amount: u64,
    token_b_amount: u64,
) -> u64 {
        let x = token_a_amount;
        let y = token_b_amount;
        let x_total = 0u64;
        let y_total = 0u64;
        let lp_total = 0u64;

        let x = PreciseNumber::new(x as u128).unwrap();
        let y = PreciseNumber::new(y as u128).unwrap();
        let min_liquidity = PreciseNumber::new(MIN_LIQUIDITY as u128).unwrap();

        // sqrt(x * y) - min_liquidity
        sqrt_precise(&x.checked_mul(&y).unwrap())
            .unwrap()
            .checked_sub(&min_liquidity)
            .unwrap()
            .floor()
            .unwrap()
            .to_imprecise()
            .unwrap() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(add(2,2), result);
    }

    #[test]
    fn calculate_first_deposit_lp_tokens_to_mint_test() {
        let result:u64 = 1238326078;
        assert_eq!(calculate_first_deposit_lp_tokens_to_mint(6000000, 255575287200), result);
    }
}
