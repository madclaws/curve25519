defmodule Inv do
  import Bitwise

  @prime 57896044618658097711785492504343953926634992332820282019728792003956564819949

  def mul_inv(_x, 0, result \\ 1)
  def mul_inv(_x, 0, result), do: result
  def mul_inv(x, exp, result) do
    IO.inspect(x, label: "x")
    IO.inspect(result, label: "result")
    if band(exp, 1) == 1 do
      result = result * x
      x = rem(x * x, @prime)
      mul_inv(x, bsr(exp, 1), result)
    else
      x = rem(x * x, @prime)
      mul_inv(x, bsr(exp, 1), result)
    end
  end
end
