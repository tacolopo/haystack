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
