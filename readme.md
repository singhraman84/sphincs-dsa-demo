# Sphincs-DSA Demo

This project demonstrates **SLH-DSA (Sphincs+) digital signature algorithm (DSA)** in Rust with support for all 12 variants. It provides key generation, message signing, and signature verification for all 12 variants.

---

## Features

- Generate key pairs for all 12 variants.
- Sign message.
- Verify signatures using public key.
- Display key parameters like key gen time, sign time, public key size, signature size.
- Used the purecrypto v0.6.25 library. The library can be checked at: https://crates.io/crates/purecrypto

---


## Usage


2. **Clone the Repository**:

Open the terminal and run:

*git clone https://github.com/singhraman84/sphincs-dsa-demo*


2. **Build the project**:

In the root directory (sphincs-dsa-demo), open the terminal and run:

*cargo build*


3. **Run the demo interactively**:

To run the project, execute the instruction:

For determinism check: This will display public keys and sha2 hash of signatures. If same seed and same message provided, public key and signature should be same. Some applications like Blockchain require this characteristics. 

*cargo run --bin signatures*


For checking key gen time, sign time, public key size, signature size and verification result, run:

*cargo run --bin main*

You will be prompted to:

- Enter a seed of 32 bytes. If seed is less than 32 bytes, letter 'a' appended and if the seed is longer, first 32 bytes are considered. 
- Enter a message to sign

##Dependencies
purecrypto = "0.6.18"
sha2 = "0.10"

##Notes
SLH-DSA is a post-quantum signature algorithm. 

## Disclaimer
This project is for educational and research purposes only.
It has not been audited and should not be used in production systems.

##License
This project is licensed under the Apache License 2.0.


**Example output for Signatures:**

LH-DSA Signature Determinism Checker...

Enter your seed string: thisismyseed01234567890123456789
Enter your message for signing: hi this is my message to sign

--- Running all 12 SLH-DSA variants ---

====================================================
SLH-DSA Variant: Sha2_128s
Public Key (hex):
7468697369736d79736565643031323333d4827b07c922a9b529b93e63cb81b2

Signature SHA-256:
ba2fb28b3d1d391b61d5cd8bb12fa5d6b77a8d395e7eefdf5467e2c0c5549c08
Signature size: 7856 bytes
====================================================
SLH-DSA Variant: Sha2_128f
Public Key (hex):
7468697369736d79736565643031323379630e22752c53d0a9344c68f780b131

Signature SHA-256:
234534f7c15dc72a7fa5fe19488aa6204b5bd0f43b3268d31c4ecb46f8ead760
Signature size: 17088 bytes
====================================================
SLH-DSA Variant: Sha2_192s
Public Key (hex):
7468697369736d79736565643031323334353637383930316f2005831dd47235978c77ff190a3dbea9ed7acd9d95a3ef

Signature SHA-256:
dd5d575b8ac7cabdb260da23fe8b6a8ce9633c6c2caf40c343792aa25f66b902
Signature size: 16224 bytes
====================================================
SLH-DSA Variant: Sha2_192f
Public Key (hex):
7468697369736d797365656430313233343536373839303116944d0da9ed0dfa864231030eff106ad5772e54f6e2bc55

Signature SHA-256:
fb994286a5935fd83886de4367225c3158a754fbd041635a19fa54a054c6031b
Signature size: 35664 bytes
====================================================
SLH-DSA Variant: Sha2_256s
Public Key (hex):
7468697369736d79736565643031323334353637383930313233343536373839af6455a229ac8a844a76ef7c4da936d8a660c346c10f30ca97ed417af1ee645f

Signature SHA-256:
1f37102a67353e5cf06512d2e49cc40fb0fbd484ba83a6f266bf473ae7da07b9
Signature size: 29792 bytes
====================================================
SLH-DSA Variant: Sha2_256f
Public Key (hex):
7468697369736d797365656430313233343536373839303132333435363738397c7e76bf6a00f594f9c549e0113cf7e60a9a61cbe65183cb78059947139cf6a1

Signature SHA-256:
2e7ee02e7b58f8133411e41e8e97443833143dcdab45dfaafe6baa3f656b1962
Signature size: 49856 bytes
====================================================
SLH-DSA Variant: Shake_128s
Public Key (hex):
7468697369736d7973656564303132338cebaed7ccf90caf9174bc0392b1e532

Signature SHA-256:
8dc82e7beb1cd8fdff54dd7d5f8f3088eacf19fcf5a9cafc03097d9954fa8500
Signature size: 7856 bytes
====================================================
SLH-DSA Variant: Shake_128f
Public Key (hex):
7468697369736d7973656564303132338da28293e8d1feaea852d96969d8011a

Signature SHA-256:
0284c3654a2e308e9cac91cd4c2d8a3a7c0f1637bc4bafb353eb0b892cb23c33
Signature size: 17088 bytes
====================================================
SLH-DSA Variant: Shake_192s
Public Key (hex):
7468697369736d7973656564303132333435363738393031736540298543a3457a67b9d879daeb74cb92079e5cc0fa20

Signature SHA-256:
cd3edec9841c526d14b350be8054e872f7f95b8baa07e6075db86a6600a876e6
Signature size: 16224 bytes
====================================================
SLH-DSA Variant: Shake_192f
Public Key (hex):
7468697369736d79736565643031323334353637383930319372668bd797cdeeacaadabca27e6a3ae6d4284bd14608a4

Signature SHA-256:
1a7f44f9e744c6cc524f8a14ecf8d24a865a93c440438dce09a752bff1d4fa54
Signature size: 35664 bytes
====================================================
SLH-DSA Variant: Shake_256s
Public Key (hex):
7468697369736d797365656430313233343536373839303132333435363738391186d035e53f5f52175cab489134efcc998f8d6531ef8e238aef10da98f84da8

Signature SHA-256:
0669cfd4e264965377639107accefc2a102365fdd5407e1685cae34b4b4e8413
Signature size: 29792 bytes
====================================================
SLH-DSA Variant: Shake_256f
Public Key (hex):
7468697369736d79736565643031323334353637383930313233343536373839031056d3b9879c2bc2fead9b650f2e75f3da111fd6acde258d9c5d8f940e9ac3

Signature SHA-256:
720095f0b0b25c6f5a4eab41244961e08d4d002d1cdba25ebd9edbc965fc7294
Signature size: 49856 bytes


**Example output for Main:**

Enter your seed string: thisismyseed01234567890123456789
Enter your message for signing: hi this is my message to sign
==============================
SLH-DSA Variant       : Sha2_128s
Security parameter n             : 16
PK size       : 32 bytes
Sig size      : 7856 bytes
Keygen time   : 2.519057584s
Sign time     : 17.66199523s
Verify result : true
==============================
SLH-DSA Variant       : Sha2_128f
Security parameter n             : 16
PK size       : 32 bytes
Sig size      : 17088 bytes
Keygen time   : 47.392738ms
Sign time     : 795.754023ms
Verify result : true
==============================
SLH-DSA Variant       : Sha2_192s
Security parameter n             : 24
PK size       : 48 bytes
Sig size      : 16224 bytes
Keygen time   : 1.423904945s
Sign time     : 13.809348233s
Verify result : true
==============================
SLH-DSA Variant       : Sha2_192f
Security parameter n             : 24
PK size       : 48 bytes
Sig size      : 35664 bytes
Keygen time   : 22.752045ms
Sign time     : 632.792426ms
Verify result : true
==============================
SLH-DSA Variant       : Sha2_256s
Security parameter n             : 32
PK size       : 64 bytes
Sig size      : 29792 bytes
Keygen time   : 909.34466ms
Sign time     : 12.562713069s
Verify result : true
==============================
SLH-DSA Variant       : Sha2_256f
Security parameter n             : 32
PK size       : 64 bytes
Sig size      : 49856 bytes
Keygen time   : 68.225168ms
Sign time     : 1.280011338s
Verify result : true
==============================
SLH-DSA Variant       : Shake_128s
Security parameter n             : 16
PK size       : 32 bytes
Sig size      : 7856 bytes
Keygen time   : 6.422162871s
Sign time     : 50.222478956s
Verify result : true
==============================
SLH-DSA Variant       : Shake_128f
Security parameter n             : 16
PK size       : 32 bytes
Sig size      : 17088 bytes
Keygen time   : 116.848514ms
Sign time     : 2.178790374s
Verify result : true
==============================
SLH-DSA Variant       : Shake_192s
Security parameter n             : 24
PK size       : 48 bytes
Sig size      : 16224 bytes
Keygen time   : 9.283216844s
Sign time     : 84.735949409s
Verify result : true
==============================
SLH-DSA Variant       : Shake_192f
Security parameter n             : 24
PK size       : 48 bytes
Sig size      : 35664 bytes
Keygen time   : 150.373519ms
Sign time     : 3.568178912s
Verify result : true
==============================
SLH-DSA Variant       : Shake_256s
Security parameter n             : 32
PK size       : 64 bytes
Sig size      : 29792 bytes
Keygen time   : 6.393442936s
Sign time     : 82.964213148s
Verify result : true
==============================
SLH-DSA Variant       : Shake_256f
Security parameter n             : 32
PK size       : 64 bytes
Sig size      : 49856 bytes
Keygen time   : 418.064576ms
Sign time     : 9.061560418s
Verify result : true
    
