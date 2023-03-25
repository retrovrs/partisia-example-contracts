/// Perform a zk computation on secret-shared data.
/// Finds the highest bidder and the amount of the second-highest bid
use pbc_zk::*;

pub fn zk_compute() -> (Sbi32, Sbi32) {
    // Initialize state
    let mut highest_bidder: Sbi32 = Sbi32::from(load_metadata::<i32>(1));
    let mut highest_amount: Sbi32 = Sbi32::from(0);
    let mut second_highest_amount: Sbi32 = Sbi32::from(0);

    // Determine max
    for variable_id in 1..(num_secret_variables() + 1) {
        if load_sbi::<Sbi32>(variable_id) > highest_amount {
            second_highest_amount = highest_amount;
            highest_amount = load_sbi::<Sbi32>(variable_id);
            highest_bidder = Sbi32::from(load_metadata::<i32>(variable_id));
        } else if load_sbi::<Sbi32>(variable_id) > second_highest_amount {
            second_highest_amount = load_sbi::<Sbi32>(variable_id);
        }
    }

    // Return highest bidder index, and second highest amount
    (highest_bidder, second_highest_amount)
}
