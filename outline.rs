// 1. The user calls a deposit function, passing in the amount they want to deposit and their address.
// 2. The contract stores the user's address and the deposited amount in its state.
// 3. When the user wants to withdraw their funds, they call a withdraw function, passing in their address.
// 4. The contract looks up the user's address in its state and retrieves the stored amount.
// 5. The contract creates a CoinJoin transaction with multiple inputs and outputs, including the user's input and output.
// 6. The contract broadcasts the CoinJoin transaction to the network and updates its state to reflect the withdrawal.

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BankMsg {
    pub from_address: HumanAddr,
    pub to_address: HumanAddr,
    pub amount: Vec<Coin>,
}

//to 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BankMsg {
    pub input_addresses: Vec<HumanAddr>,
    pub output_addresses: Vec<HumanAddr>,
    pub amounts: Vec<Coin>,
}


//The amounts field would then contain a list of the amounts being sent to each output address,
//with a corresponding entry in the output_addresses list. The input_addresses list would contain
// the addresses of the inputs being spent in the transaction.

//To handle these multiple inputs and outputs in the contract logic, you would need to modify 
//the code to iterate over the lists of input and output addresses and amounts, and create the 
//appropriate messages for each. For example, you might do something like this:

let mut messages = Vec::new();

for (input_address, output_address, amount) in input_addresses.zip(output_addresses).zip(amounts) {
    let bank_msg = BankMsg::Send {
        from_address: input_address,
        to_address: output_address,
        amount: vec![amount],
    };
    messages.push(bank_msg);
}

let resp = Response::new()
    .add_messages(messages)
    .add_attribute("action", "coinjoin")
    .add_attribute("total amount", total_amount.to_string());
Ok(resp)
