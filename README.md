<h1>ICP Based Token Wallet</h1>

This is a basic implementation of a wallet on the Internet Computer (IC) that supports token management functionalities like initialization, checking wallet status, receiving tokens, sending tokens, and checking token balances.

Features
 --Wallet Initialization: Initializes a wallet for a user.
 --Wallet Status: Checks if the wallet is initialized.
 --Token Balance: Retrieves the balance of a specific token.
 --Receive Tokens: Adds tokens to the wallet.
 --Send Tokens: Sends tokens from the wallet.

Functions
  1. init_wallet()
  This function initializes a new wallet. It does not take any arguments and creates an empty wallet.

Command:
  dfx canister call example_backend init_wallet

  2. get_wallet_status(user_address: String) -> String
  This function returns the status of the wallet for a specific user address. It checks if the wallet for the given user address is initialized.

Command: 
  dfx canister call example_backend get_wallet_status '("user_address")'
Arguments:
  user_address: The address of the user whose wallet status is to be checked (string).
Returns:
  "Wallet initialized": If the wallet has been initialized for the given user.
  "Wallet not initialized": If the wallet has not been initialized.

  3. get_balance(token: String) -> u64
  This function retrieves the balance of a specific token for the current user. If no balance is found, it returns 0.

Command:
  dfx canister call example_backend get_balance '("TokenName")'
Arguments:
  token: The name of the token whose balance is to be checked (string).
Returns:
  The balance of the specified token.

  4. receive_tokens(token: String, amount: u64) -> String
  This function allows a user to receive tokens. The specified amount is added to the user's wallet for the given token.

Command:
  dfx canister call example_backend receive_tokens '("TokenName", 100)'
Arguments:
  token: The name of the token to receive (string).
  amount: The amount of the token to be added to the wallet (u64).
Returns:
  "Received {amount} tokens of {token}": If tokens were successfully received.
  "Wallet not initialized": If the wallet is not initialized.

  5. send_tokens(token: String, amount: u64) -> String
  This function allows a user to send tokens from their wallet. It deducts the specified amount of the given token from the wallet.

Command:
  dfx canister call example_backend send_tokens '("TokenName", 50)'
Arguments:
  token: The name of the token to send (string).
  amount: The amount of the token to be sent (u64).
Returns:
  "Sent {amount} tokens of {token}": If the tokens were successfully sent.
  "Insufficient balance": If the user does not have enough tokens in their wallet.
  "Wallet not initialized": If the wallet is not initialized.

Prerequisites
DFX: Ensure you have DFX installed and set up to deploy and manage your canisters.
Install it by following the instructions from DFINITY SDK.

Internet Computer Replica: The canister is intended to be deployed on the Internet Computer. Ensure your local replica is running with: dfx start

Local Development
  1. Deploy the Canister
  To deploy the canister, first build and deploy the canister for backend only with the following commands:

  dfx deploy example_backend 
  This will compile and deploy the canister to the local replica.

  2. Interact with the Canister
  Use the commands provided above to interact with the deployed canister. Each function is invoked via the dfx canister call command.  


