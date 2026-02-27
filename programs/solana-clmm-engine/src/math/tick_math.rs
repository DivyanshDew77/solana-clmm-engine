use crate::math::u256_math::*;
use std::convert::TryInto;

// Unlike Uniswap V3 (which uses a Q64.96 format), we use a Q64.64 format.
pub const MAX_SQRT_PRICE_X64: u128 = 79226673515401279992447579055;
pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;

const LOG_B_2_X32: i128 = 59543866431248i128; // logb(2) in x32 precision (b = (1.0001)^(1/2))
const BIT_PRECISION: u32 = 14; // for accuracy, we use 14 bits for the decimal part
const LOG_B_P_ERR_MARGIN_LOWER_X64: i128 = 184467440737095516i128;
const LOG_B_P_ERR_MARGIN_UPPER_X64: i128 = 184467440737095516i128;

/// Threshold (2^15): If tick_spacing is greater than this value, the pool disables concentrated liquidity
/// and acts as a standard Full-Range AMM
pub const FULL_RANGE_ONLY_TICK_SPACING_THRESHOLD: u16 = 32768; 

/// Derive the sqrt-price from a tick index. The precision of this method is only guaranteed 
/// if tick is within the bounds of {max,min} tick-index.
/// 
/// # Parameters
/// - 'tick' - A i32 integer representing the tick integer
/// 
/// # Returns
/// - 'Ok' : A u128 Q32.64 representing the sqrt_price
pub fn sqrt_price_from_tick_index(tick: i32) -> u128 {
    // ------------------------------------------------------------------------
    // TODO: Implement the bitwise If statements for the 1.0001^i calculation
    // Requires setting up a custom U256 math struct first to prevent u128 overflow 
    // during intermediate multiplication steps.
    // 
    // Status: Paused core math implementation for College Mid-Terms. 
    // Will resume development on March 3rd.
    // ------------------------------------------------------------------------
    todo!("Bitwise tick-to-price math pending U256 integration.")
}