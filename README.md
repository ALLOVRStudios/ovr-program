<p align="center">
    <img src="https://raw.githubusercontent.com/ALLOVRStudios/ovr-program/main/banner.jpg" margin="auto" />
</p>

# OVR Program

OVR Program is a Solana Program which forms the basis of the ALLOVR Protocol.

Currently the only available functionality is:

- init
- mint

## Invoking functions.

The first byte of the data parameter determines the function to be executed
- 0 = init
- 1 = mint
- ...more to follow

## Init

The init function can be invoked exactly once and only by the update authority of the OVR Program (present as signer). It creates the state and mint accounts used to store data about and mint the ALL token respectively. 

The following accounts are required:
- `[writable, signer]` Initator (program_id)
- `[writable, signer]` State (PDA with seed prefix ALLOVRSTATE)
- `[writable, signer]` Payer
- `[writable, signer]` Mint (ALLOVR Mint account with known address ALLM...)
- `[]` Mint Authority (PDA with seed prefix ALLOVRMINT)
- `[]` Token Program
- `[]` Rent Sysvar
- `[]` System    

Additionaly, the data parameter must contain the 4 founder ATA addresses that will be credited with ALL tokens on the first mint call.

## Mint

Once initialised, the mint function can be executed (exactly once and only by the update authority of the OVR Program). Upon execution 100 000 000 ALL tokens will be minted to 
- A treasury account > 70% (70 000 000)
- Founder 1 account > 7.5% (7 500 000)
- Founder 2 account > 7.5% (7 500 000)
- Founder 3 account > 7.5% (7 500 000)
- Founder 4 account > 7.5% (7 500 000)

The following accounts are required:
- `[signer]` Initator (program_id)
- `[writable]` State (PDA with seed prefix ALLOVRSTATE)
- `[writable, signer]` Payer
- `[writable]` Mint (ALLOVR Mint account with known address ALLM...)
- `[]` Mint Authority (PDA with seed prefix ALLOVRMINT)
- `[writable]` Treasury Token Account
- `[writable]` Founder 1 Token Account
- `[writable]` Founder 2 Token Account
- `[writable]` Founder 3 Token Account
- `[writable]` Founder 4 Token Account
- `[]` Token Program
- `[]` Rent Sysvar
- `[]` Clock Sysvar
- `[]` System    

## Known addresses

pub const ALLOVR_PROGRAM_ID: &'static str = "B6w8UQGNEbujVCvtdMhsEPfjnxd3w8MgiMTx6syAu123";
pub const ALLOVR_STATE_ID: &'static str = "ALLSsF2ZPrXLSBerAGZoig9nRLPb9sVsWcnM7j3u6JfR";
pub const ALLOVR_MINT_ID: &'static str = "ALLMusFNnKAjg5QdbmcSxPsseERfUb4WFvvxU6zaR337";

|Account|Address                                     |
|-------|--------------------------------------------|
|Program|B6w8UQGNEbujVCvtdMhsEPfjnxd3w8MgiMTx6syAu123|
|State  |ALLSsF2ZPrXLSBerAGZoig9nRLPb9sVsWcnM7j3u6JfR|
|Mint   |ALLMusFNnKAjg5QdbmcSxPsseERfUb4WFvvxU6zaR337|

## Build

    cargo build-bpf

## Clean

    cargo clean

## Test

    cargo test-bpf

More coming soon.