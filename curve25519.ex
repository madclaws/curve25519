defmodule Curve25519 do
  @moduledoc """
  A simple curve25519 implementation
  """
  import Bitwise

  @prime 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED
  @k_and 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8
  @k_or 0x4000000000000000000000000000000000000000000000000000000000000000
  @u_and 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
  @all256 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
  @a24 121665

  def test_k_u_iter(k, _u, 0), do: IO.inspect(k)

  def test_k_u_iter(k, u, iter) do
    k1 = mul_k_u(k, u)
    u1 = k
    test_k_u_iter(k1, u1, iter - 1)
  end

  def test_echd() do
    g = 9
    alice_pri_key = :rand.uniform(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) |> IO.inspect(label: :alice_pkey)
    bob_pri_key = :rand.uniform(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) |> IO.inspect(label: :bob_pkey)

    alice_pub_key = mul_k_u(alice_pri_key, g) |> IO.inspect(label: :alice_pub_key)
    bob_pub_key = mul_k_u(bob_pri_key, g) |> IO.inspect(label: :bob_pub_key)

    alice_shared_Key = mul_k_u(alice_pri_key, bob_pub_key) |> IO.inspect(label: :alice_shared_Key)
    bob_shared_Key = mul_k_u(bob_pri_key, alice_pub_key) |> IO.inspect(label: :bob_shared_Key)

    IO.inspect(alice_shared_Key)

    iv = :crypto.strong_rand_bytes(16)

    enc_text = :crypto.crypto_one_time(:aes_128_ctr, <<:binary.decode_unsigned(<<alice_shared_Key::size(128)>>, :little)::size(128)>>, iv, <<"hello bob">>, true)

    # decrypting
    :crypto.crypto_one_time(:aes_128_ctr, <<:binary.decode_unsigned(<<bob_shared_Key::size(128)>>, :little)::size(128)>>, iv, enc_text, false)

  end

  ########### Basic modular operations ###############
  @spec add(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  def add(x, y) do
    (x + y) |> rem(@prime)
  end

  @spec mul(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  def mul(x, y) do
    (x * y) |> rem(@prime)
  end

  @spec mul(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  def sub(x, y) do
    x + (@prime - y) |> rem(@prime)
  end

  @spec mul(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  defp inv(x) do
    calculate_exponent(x, @prime - 2)
  end

  #This is exponentation by squaring

  # say we have to calculate x^exp,
  # - let result = 1,
  # - we go from LSB of the exp to MSB
  #    - the way we do this is by right shifting exp on every iteration and reassigning new value of exp as it.
  # - we do bitwise AND, to find out if the current LSB is 1 or not.
  # - If 1, then we multiply x with `result`
  # - Then we do x = x * x
  # - This repeats until exp is 0 .
  @spec calculate_exponent(non_neg_integer(), non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  defp calculate_exponent(x, exp, result \\ 1)
  defp calculate_exponent(_x, 0, result), do: result
  defp calculate_exponent(x, exp, result) do
    if band(exp, 1) == 1 do
      result = result * x
      x = rem(x * x, @prime)
      calculate_exponent(x, bsr(exp, 1), result)
    else
      x = rem(x * x, @prime)
      calculate_exponent(x, bsr(exp, 1), result)
    end
  end



  ##### Elliptic curve operations ##########################
  # f(k, u) -> u, another point's u
  @spec mul_k_u(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  defp mul_k_u(k, u) do
    k1 = :binary.decode_unsigned(<<k::size(256)>>, :little)
    k1 = band(k1, @k_and) |> bor(@k_or)
    u1 = :binary.decode_unsigned(<<u::size(256)>>, :little)
    u1 = band(u1, @u_and)
    mul_k_u(254, k1, u1, 1, 0, u1, 1, 0)
  end

  defp mul_k_u(t, _k, _x_1, x_2, z_2, x_3, z_3, swap) when t == -1 do
    {x_2a, _x_3a} = cswap(swap, x_2, x_3)
    {z_2a, _z_3a} = cswap(swap, z_2, z_3)
    inverse = inv(z_2a)
    result = mul(x_2a, inverse)
    :binary.decode_unsigned(<<result::size(256)>>, :little)
  end

  defp mul_k_u(t, k, x_1, x_2, z_2, x_3, z_3, swap) do
    k_t = bsr(k, t) |> band(1)
    swap_a = bxor(swap, k_t)
    {x_2a, x_3a} = cswap(swap_a, x_2, x_3)
    {z_2a, z_3a} = cswap(swap_a, z_2, z_3)
    swap_b = k_t
    a = add(x_2a, z_2a)
    aa = mul(a, a)
    b = sub(x_2a, z_2a)
    bb = mul(b, b)
    e  = sub(aa, bb)
    c = add(x_3a, z_3a)
    d = sub(x_3a, z_3a)
    da = mul(d, a)
    cb = mul(c, b)
    xx1 = add(da, cb)
    x_3b = mul(xx1, xx1)
    xx2 = sub(da, cb)
    xx3 = mul(xx2, xx2)
    z_3b = mul(x_1, xx3)
    x_2b = mul(aa, bb)
    xx4 = mul(@a24, e)
    xx5 = add(aa, xx4)
    z_2b = mul(e, xx5)
    mul_k_u(t - 1, k, x_1, x_2b, z_2b, x_3b, z_3b, swap_b)
  end

  # conditional swap
  defp cswap(swap, x2, x3) do
    dummy = swap * band(@all256, bxor(x2, x3))
    x2a = bxor(x2, dummy)
    x3a = bxor(x3, dummy)
    {x2a, x3a}
  end
end
