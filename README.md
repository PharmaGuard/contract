# PharmaGuard

A blockchain-based decentralized platform designed to enhance the traceability and transparency of pharmaceutical products. 
By leveraging Web3 and IoT technology, PharmaGuard records, tracks, and visualizes real-time data related to medications throughout their lifecycle. 

# Architecture diagram
![image](https://github.com/user-attachments/assets/5bfee6c5-38e4-4f14-8106-6071725f6207)

# Version requirements
```
anchor-lang = "0.30.1"
anchor-spl = "0.30.1"
spl-token = "4.0.3"
spl-token-2022 = "3.0.4"
solana-program = "1.18.0"
```
# Deployment situation
1. CirTw1apKgQHSveAvpMp3P283Z54XzJWTBy1LHVLz7E9 (devnet) (Note: Verification has not been successful yet)
# Instruction Introduction
### 1. `initialUserPaa`
Initializes the User Pharmacy Associated Account, linking a user to their pharmacy.
### 2. `initialPharmacy`
Sets up the Pharmacy Info Account, storing essential details about the pharmacy.
### 3. `bindPharmacy`
Associates the pharmacy information with the user's pharmacy account, establishing a connection between the two.
### 4. `initialDrug`
Creates the Drug Account to store information about a specific drug, including its price and storage conditions.
### 5. `takeoutOrder`
Enables users to place orders for drugs from the pharmacy, initiating the order process.
### 6. `sendOut`
Allows the pharmacy to dispatch the ordered drugs to the user.
### 7. `signFor`
Facilitates the user to sign for their received order, confirming the delivery.
