# Curve25519

> A toy implementation of curve25519 in Rust and Elixir

- Implementation of modular arithmetic functions.
- Implementation of montogomery ladder algorithm for scalar multiplication in elliptic curves
- Generates elliptic curve key-pair.
- Uses the key-pair to generate Diffie Hellman exchange key.
- Uses the Diffie Hellman key to encrypt and decrypt the message

## Running

### Rust

```
cargo run
```

### Elixir

```elixir
# open shell
iex

# compile the elixir file
c("curve25519.ex")

# run the demo function

Curve25519.test_echd

```

```
> cargo run

Curve25519

Prime modulus used = "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed"

ALICE PRIVATE KEY - 91796755515922421697872634867622695009096075288433492651354928529005968778609
ALICE PUBLIC KEY - 28559537023499553536216910445193515752868334340426334669298742073534077485387

BOB PRIVATE KEY - 79717216449833096316586654909324572140073409042408235159214890408562361918188
BOB PUBLIC KEY - 86217167649832879522529786834996057469919958003771685940603501201957816637503

ALICE SHARED KEY - 92239961839722010831969999144649500625489378148050776162123999028101498930260
BOB SHARED KEY - 92239961839722010831969999144649500625489378148050776162123999028101498930260

ALICE encrypted `Hello bob` into - [184, 178, 117, 132, 135, 89, 207, 57, 3]

BOB decrypted [184, 178, 117, 132, 135, 89, 207, 57, 3] into "Hello bob"

```